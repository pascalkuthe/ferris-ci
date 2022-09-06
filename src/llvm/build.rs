use std::fs;

use xshell::{cmd, Shell};

use crate::flags::Build;
use crate::llvm::{BUILD_DIR, SRC_DIR};

impl Build {
    pub fn run_llvm(&self, sh: &Shell) -> anyhow::Result<()> {
        const URL: &str = "https://github.com/llvm/llvm-project";
        let tag = format!("llvmorg-{}", self.version);
        self.get_src(sh, URL, SRC_DIR, &tag)?;
        self.configure_llvm(sh)?;
        self.build_llvm(sh)?;
        self.move_files(sh)?;
        Ok(())
    }

    fn enable_assert(&self) -> &'static str {
        match self.debug {
            true => "ON",
            false => "OFF",
        }
    }

    fn config(&self) -> &'static str {
        match self.debug {
            true => "RelWithDebInfo",
            false => "Release",
        }
    }

    fn configure_llvm(&self, sh: &Shell) -> anyhow::Result<()> {
        let current_dir = sh.current_dir();
        let src_dir = current_dir.join(SRC_DIR).join("llvm");
        let enable_assert = self.enable_assert();

        // xshell freezes on windows for some reason
        let build_dir = sh.current_dir().join(BUILD_DIR);
        if build_dir.exists() {
            fs::remove_dir_all(build_dir)?;
        }

        let cmake = if cmd!(sh, "cmake3 --version").run().is_ok() {
            "cmake3"
        } else {
            "cmake"
        };
        let cmd = cmd!(
            sh,
            "{cmake} -GNinja
                -S {src_dir}
                -B {BUILD_DIR} 
                -DLLVM_PARALLEL_LINK_JOBS=1
                -DLLVM_TARGETS_TO_BUILD=AArch64;ARM;X86
                -DLLVM_OPTIMIZED_TABLEGEN=ON
                -DCMAKE_BUILD_TYPE=Release
                -DLLVM_ENABLE_LIBXML2=OFF
                -DLLVM_ENABLE_ZLIB=OFF
                -DLLVM_ENABLE_ASSERTIONS={enable_assert}
                -DLLVM_BUILD_TOOLS=OFF
                -DLLVM_BUILD_EXAMPLES=OFF
                -DLLVM_BUILD_TOOLS=OFF
                -DLLVM_BUILD_RUNTIME=OFF
                -DLLVM_ENABLE_BINDINGS=OFF
                -DLLVM_INSTALL_UTILS=OFF
                -DLLVM_LIBDIR_SUFFIX=64
                -DLLVM_ENABLE_PROJECTS=clang;lld
                -DCLANG_ENABLE_ARCMT=OFF
                -DCLANG_ENABLE_STATIC_ANALYZER=OFF
                -DLLVM_BUILD_LLVM_C_DYLIB=OFF"
        );

        let cmd = if cfg!(unix) {
            cmd.args(["-DLLVM_ENABLE_LLD=ON"])
        } else {
            cmd.args([
                "-DCMAKE_C_COMPILER:PATH=clang-cl.exe",
                "-DCMAKE_CXX_COMPILER:PATH=clang-cl.exe",
                "-DCMAKE_RC_COMPILER:PATH=llvm-rc.exe",
                "-DCMAKE_LINKER:PATH=lld-link.exe",
                "-DLLVM_ENABLE_DIA_SDK=OFF",
            ])
        };

        cmd.run()?;
        Ok(())
    }

    fn build_llvm(&self, sh: &Shell) -> anyhow::Result<()> {
        let config = self.config();

        let cmake = if cmd!(sh, "cmake3 --version").run().is_ok() {
            "cmake3"
        } else {
            "cmake"
        };

        #[cfg(unix)]
        let tools = ["llvm-cov", "llvm-profdata", "llvm-config"];
        #[cfg(windows)]
        let tools = ["llvm-config"];

        for tool in tools {
            let cmd = cmd!(
                sh,
                "{cmake} --build {BUILD_DIR} --target {tool} --config {config}"
            );
            self.add_parallel_flag(cmd).run()?
        }
        let cmd = cmd!(sh, "{cmake} --build {BUILD_DIR} --config {config}");

        self.add_parallel_flag(cmd).run()?;

        Ok(())
    }

    fn move_files(&self, sh: &Shell) -> anyhow::Result<()> {
        let dst = sh.current_dir().join(BUILD_DIR).join("include");
        let tmp = sh.current_dir().join(BUILD_DIR).join("include_back");
        fs::rename(&dst, &tmp)?;

        fs::rename(
            sh.current_dir().join(SRC_DIR).join("llvm").join("include"),
            &dst,
        )?;

        for dir in ["Config", "IR", "Support"] {
            let dst = dst.join("llvm").join(dir);
            let dir = tmp.join("llvm").join(dir);
            for file in fs::read_dir(dir)? {
                let file = file?;
                if let Some(name) = file.file_name().to_str() {
                    if name.ends_with(".h") || name.ends_with(".inc") {
                        fs::rename(file.path(), dst.join(file.file_name()))?;
                    }
                }
            }
        }

        fs::rename(
            sh.current_dir()
                .join(SRC_DIR)
                .join("lld")
                .join("include")
                .join("lld"),
            &dst.join("lld"),
        )?;

        let src_dir = sh.current_dir().join(SRC_DIR);
        if src_dir.exists() {
            fs::remove_dir_all(src_dir)?;
        }
        Ok(())
    }
}
