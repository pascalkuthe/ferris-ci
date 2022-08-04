use std::fs;

use xshell::{cmd, Cmd, Shell};

use crate::flags::{Archive, Build};

impl Build {
    pub fn run(&self, sh: &Shell) -> anyhow::Result<()> {
        match self.program {
            crate::KnownProgram::LLVM => self.run_llvm(sh)?,
            crate::KnownProgram::Wine => self.run_wine(sh)?,
            crate::KnownProgram::Git => self.run_git(sh)?,
        }

        if self.upload {
            Archive {
                program: self.program,
                version: self.version.clone(),
                upload: true,
                no_save: true,
                env_access_key: self.env_access_key,
                src_dir: None,
                debug: self.debug,
            }
            .run(sh)?;
        }
        Ok(())
    }

    pub fn get_src(&self, sh: &Shell, url: &str, src_dir: &str, tag: &str) -> anyhow::Result<()> {
        let src_dir = sh.current_dir().join(src_dir);
        if src_dir.exists() {
            fs::remove_dir_all(&src_dir)?;
        }
        cmd!(
            sh,
            "git clone --depth 1 --single-branch --branch {tag} {url} {src_dir}"
        )
        .run()?;
        Ok(())
    }

    pub fn add_parallel_flag<'a>(&self, mut cmd: Cmd<'a>) -> Cmd<'a> {
        if let Some(parallel) = self.parallel {
            cmd = cmd.arg("-j").arg(parallel.to_string());
        }
        cmd
    }

    pub fn clfags(&self) -> &'static str {
        if self.debug {
            "-ggdb -m64 -Os"
        } else {
            "-m64 -Os"
        }
    }

    pub fn ldfags(&self) -> &'static str {
        if self.debug {
            "-ggdb -m64"
        } else {
            "-m64"
        }
    }

    pub fn add_cc_flags<'a>(&self, cmd: Cmd<'a>) -> Cmd<'a> {
        cmd.envs([("CFLAGS", self.clfags()), ("LDFLAGS", self.ldfags())])
    }
}
