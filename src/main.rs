use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::bail;
use xshell::Shell;

mod archive;
mod build;
mod download;
mod flags;
mod install_rust;
mod install_tool;
mod llvm;
// mod release;
mod git;
mod s3;
mod vendor;
mod wine;

pub const TARGET: &str = env!("CFG_COMPILER_HOST_TRIPLE");

fn main() -> anyhow::Result<()> {
    let sh = &Shell::new()?;
    if let Some(root) = project_root() {
        sh.change_dir(root);
    }

    match flags::FerrisCi::from_env()?.subcommand {
        flags::FerrisCiCmd::Help(_) => {
            println!("{}", flags::FerrisCi::HELP);
            Ok(())
        }
        flags::FerrisCiCmd::Upload(cmd) => cmd.run(),
        flags::FerrisCiCmd::InstallRust(cmd) => cmd.run(sh),
        flags::FerrisCiCmd::InstallTool(cmd) => cmd.run(sh),
        flags::FerrisCiCmd::Build(cmd) => cmd.run(sh),
        flags::FerrisCiCmd::Archive(cmd) => cmd.run(sh),
        flags::FerrisCiCmd::Download(cmd) => cmd.run(sh),
        flags::FerrisCiCmd::InstallLlvmBuildDeps(_) => todo!(),
        flags::FerrisCiCmd::Release(_) => todo!(),
        flags::FerrisCiCmd::Vendor(cmd) => cmd.subcommand.run(sh),
    }
}

fn project_root() -> Option<PathBuf> {
    let res = Path::new(&env::var("CARGO_MANIFEST_DIR").ok()?)
        // .ancestors()
        // .nth(1)
        // .unwrap()
        .to_path_buf();
    Some(res)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ArchiveFormat {
    TarZst,
    TarGz,
    // ZipDeflate,
}

impl FromStr for ArchiveFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let algorithm = match &*s.to_lowercase() {
            "tar-gz" => Self::TarGz,
            "tar-zst"|"tar-zstd" => Self::TarZst,
            // "zip" => Self::ZipDeflate,
            alg => bail!("Unkown compression algorithm '{alg}': Candidates are 'tar-zst', 'tar-zstd' and 'tar-gz'"),
        };
        Ok(algorithm)
    }
}

// fn detected_docker(sh: &Shell) -> anyhow::Result<&'static str> {
//     match cmd!(sh, "docker --version").read() {
//         Ok(version) => {
//             println!("found docker ({version})");
//             return Ok("docker");
//         }
//         Err(_) => {
//             println!("docker` was not found!")
//         }
//     }

//     match cmd!(sh, "podman --version").read() {
//         Ok(version) => {
//             println!("found podman ({version})");
//             Ok("podman")
//         }
//         Err(_) => {
//             bail!("found neither podman nor docker! Please make sure they are in your path")
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KnownProgram {
    LLVM,
    Wine,
    Git,
}

impl FromStr for KnownProgram {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let program = s.to_lowercase();
        match &*program {
            "llvm" => Ok(Self::LLVM),
            "wine" => Ok(Self::Wine),
            "git" => Ok(Self::Git),
            _ => bail!("unkown program '{program}'; candidates are 'llvm' and 'wine'"),
        }
    }
}

pub fn archive_name(name: &str, version: &str, debug: bool) -> String {
    archive_name_with_target(name, version, debug, None)
}

pub fn archive_name_with_target(
    name: &str,
    version: &str,
    debug: bool,
    target: Option<&str>,
) -> String {
    let postfix = match debug {
        true => "ON",
        false => "OFF",
    };
    let target = target.unwrap_or(TARGET);
    format!("{name}-{version}-{target}-{postfix}.tar.zst")
}

impl KnownProgram {
    pub fn name(&self) -> &'static str {
        match self {
            KnownProgram::LLVM => "llvm",
            KnownProgram::Wine => "wine",
            KnownProgram::Git => "git",
        }
    }
    pub fn archive_name(self, version: &str, debug: bool) -> String {
        self.archive_name_with_target(version, debug, None)
    }
    pub fn archive_name_with_target(
        self,
        version: &str,
        debug: bool,
        target: Option<&str>,
    ) -> String {
        let postfix = match debug {
            true => "ON",
            false => "OFF",
        };
        let name = self.name();
        let target = target.unwrap_or(TARGET);
        format!("{name}-{version}-{target}-{postfix}.tar.zst")
    }

    pub fn build_dir(self) -> String {
        format!("{}_build", self.name())
    }

    pub fn src_dir(self) -> String {
        format!("{}_src", self.name())
    }
}
