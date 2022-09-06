use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use indicatif::ProgressBar;
use xshell::Shell;

use crate::wine::BUILD_DIR;

pub fn populate_archive(
    src_dir: &Option<PathBuf>,
    sh: &Shell,
    dst: &mut impl Write,
    pb: &ProgressBar,
) -> anyhow::Result<()> {
    let llvm_dir = src_dir
        .clone()
        .unwrap_or_else(|| sh.current_dir().join(BUILD_DIR));
    let mut tar_builder = tar::Builder::new(dst);
    tar_builder.append_dir("WINE", &llvm_dir)?;
    let bin_dir = llvm_dir.join("bin");
    tar_builder.append_dir("WINE/bin", &bin_dir)?;
    let mut add_executable = |name: &str| -> anyhow::Result<()> {
        let path = bin_dir.join(&*name);
        let mut file = File::open(path)?;
        let new_len = file.metadata()?.len();
        pb.set_position(0);
        pb.set_length(new_len);
        let path = format!("WINE/bin/{name}");
        pb.println(&path);
        tar_builder.append_file(path, &mut file)?;
        Ok(())
    };

    add_executable("wine64")?;
    add_executable("wineboot")?;
    add_executable("winepath")?;
    add_executable("wineserver")?;
    add_executable("regsvr32")?;

    pb.set_length(241 * 1024 * 1014);
    tar_builder.append_dir_all("WINE/lib64", &llvm_dir.join("lib"))?;

    pb.set_length(11 * 1024 * 1014);
    tar_builder.append_dir("WINE/share", &llvm_dir.join("share"))?;
    tar_builder.append_dir_all("WINE/share/wine", &llvm_dir.join("share").join("wine"))?;

    Ok(())
}
