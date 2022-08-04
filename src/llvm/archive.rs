use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use indicatif::ProgressBar;
use tar::{EntryType, Header};
use xshell::{cmd, Shell};

use super::BUILD_DIR;

pub fn populate_archive(
    llvm_dir: &Option<PathBuf>,
    sh: &Shell,
    dst: &mut impl Write,
    pb: &ProgressBar,
) -> anyhow::Result<()> {
    let llvm_dir = llvm_dir
        .clone()
        .unwrap_or_else(|| sh.current_dir().join(BUILD_DIR));
    let mut tar_builder = tar::Builder::new(dst);
    tar_builder.append_dir("LLVM", &llvm_dir)?;
    let bin_dir = llvm_dir.join("bin");
    tar_builder.append_dir("LLVM/bin", &bin_dir)?;
    let mut add_executable = |name: &str| -> anyhow::Result<()> {
        #[cfg(windows)]
        let name = format!("{name}.exe");
        let path = bin_dir.join(&*name);
        let mut file = File::open(path)?;
        let new_len = file.metadata()?.len();
        pb.set_position(0);
        pb.set_length(new_len);
        let path = format!("LLVM/bin/{name}");
        pb.println(&path);
        tar_builder.append_file(path, &mut file)?;
        Ok(())
    };

    add_executable("llvm-config")?;
    #[cfg(not(windows))]
    {
        add_executable("llvm-cov")?;
        add_executable("llvm-profdata")?;
        add_executable("llvm-rc")?;
        add_executable("llvm-ar")?;
        add_executable("lld")?;
        add_executable("clang")?;
        add_executable("clang-format")?;

        let mut add_executable_link = |name: &str, src: &str| -> anyhow::Result<()> {
            let path = format!("LLVM/bin/{name}");
            let mut header = Header::new_old();
            header.set_entry_type(EntryType::Symlink);
            header.set_size(0);
            tar_builder.append_link(&mut header, path, src)?;
            Ok(())
        };

        add_executable_link("clang-cl", "clang")?;
        add_executable_link("clang++", "clang")?;
        add_executable_link("ld.lld", "lld")?;
        add_executable_link("ld64.lld", "lld")?;
        add_executable_link("lld-link", "lld")?;
        add_executable_link("llvm-lib", "llvm-ar")?;
    }

    let llvm_config = bin_dir.join("llvm-config");
    #[cfg(windows)]
    let llvm_config = llvm_config.with_extension("exe");
    let libs = find_llvm_libs(sh, &llvm_config)?;
    tar_builder.append_dir("LLVM/lib64", llvm_dir.join("lib64"))?;
    let lib_dir = llvm_dir.join("lib64");
    for lib in libs {
        let path = format!("LLVM/lib64/{lib}");
        pb.println(&path);
        let mut lib_file = File::open(lib_dir.join(&lib))?;
        pb.set_position(0);
        pb.set_length(lib_file.metadata()?.len());
        tar_builder.append_file(path, &mut lib_file)?;
    }

    pb.set_position(0);
    pb.set_length(20 * 1024 * 1024);
    pb.println("/LLVM/lib64/clang/");
    tar_builder.append_dir_all("LLVM/lib64/clang", lib_dir.join("clang"))?;

    pb.set_position(0);
    pb.set_length(20 * 1024 * 1024);
    pb.println("/LLVM/include");
    let include_dir = llvm_dir.join("include");
    tar_builder.append_dir_all("LLVM/include", &include_dir)?;
    tar_builder.finish()?;
    Ok(())
}

fn find_llvm_libs(sh: &Shell, llvm_config: &Path) -> anyhow::Result<Vec<String>> {
    let components = &["x86", "arm", "aarch64", "ipo", "bitreader", "bitwriter"];

    let output = cmd!(sh, "{llvm_config} --link-static --libs {components...}").read()?;
    let mut libs = Vec::new();
    for lib in output.split_whitespace() {
        let lib = if let Some(stripped) = lib.strip_prefix("-l") {
            format!("lib{stripped}.a")
        } else if let Some(stripped) = lib.strip_prefix('-') {
            format!("lib{stripped}.a")
        } else if Path::new(lib).exists() {
            Path::new(lib)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        } else if lib.ends_with(".lib") {
            lib.to_owned()
        } else {
            continue;
        };

        // Don't need or want this library, but LLVM's CMake build system
        // doesn't provide a way to disable it, so filter it here even though we
        // may or may not have built it. We don't reference anything from this
        // library and it otherwise may just pull in extra dependencies on
        // libedit which we don't want
        if lib == "libLLVMLineEditor" || !lib.contains("LLVM") {
            continue;
        }

        libs.push(lib);
    }

    Ok(libs)
}
