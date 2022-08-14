use crate::{ArchiveFormat, KnownProgram};
use std::path::PathBuf;

xflags::xflags! {
    src "./src/flags.rs"

    /// Run custom build command.
    cmd ferris-ci {
        default cmd help {
            /// Print help information.
            optional -h, --help
        }

        cmd build
        required program: KnownProgram
        required version: String
        {
            optional --upload
            optional --debug
            optional -j, --parallel n: u32
            optional --env-access-key
        }

        cmd archive
        required program: KnownProgram
        required version: String
        {
            // upload resulting archive
            optional --upload
            // do save resulting file instead keep in memory and upload directly, reduces io
            // overhead significantly
            optional --no-save
            optional --env-access-key
            optional --src-dir src_dir: PathBuf
            optional --debug
        }

        cmd download
        required file: String
        {
            optional --debug
            optional --base-url base_url: String
            optional --decompress
        }

        cmd install-llvm-build-deps{}


        cmd upload
        required file: PathBuf
        {
            /// get access key from enviorment var instead of prompting
            optional --env-access-key
            optional -z, --compress compression: ArchiveFormat
            optional -o, --object object: String
        }

        cmd install-rust
        required version: String
        {}


        cmd install-tool
        required name: String
        required url: String
        {}

        cmd release
        required github_token: String
        {}

        cmd vendor{
            cmd hash{}
            cmd normalize{}
            cmd generate{}
        }

    }

}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct FerrisCi {
    pub subcommand: FerrisCiCmd,
}

#[derive(Debug)]
pub enum FerrisCiCmd {
    Help(Help),
    Build(Build),
    Archive(Archive),
    Download(Download),
    InstallLlvmBuildDeps(InstallLlvmBuildDeps),
    Upload(Upload),
    InstallRust(InstallRust),
    InstallTool(InstallTool),
    Release(Release),
    Vendor(Vendor),
}

#[derive(Debug)]
pub struct Help {
    pub help: bool,
}

#[derive(Debug)]
pub struct Build {
    pub program: KnownProgram,
    pub version: String,

    pub upload: bool,
    pub debug: bool,
    pub parallel: Option<u32>,
    pub env_access_key: bool,
}

#[derive(Debug)]
pub struct Archive {
    pub program: KnownProgram,
    pub version: String,

    pub upload: bool,
    pub no_save: bool,
    pub env_access_key: bool,
    pub src_dir: Option<PathBuf>,
    pub debug: bool,
}

#[derive(Debug)]
pub struct Download {
    pub file: String,

    pub debug: bool,
    pub base_url: Option<String>,
    pub decompress: bool,
}

#[derive(Debug)]
pub struct InstallLlvmBuildDeps;

#[derive(Debug)]
pub struct Upload {
    pub file: PathBuf,

    pub env_access_key: bool,
    pub compress: Option<ArchiveFormat>,
    pub object: Option<String>,
}

#[derive(Debug)]
pub struct InstallRust {
    pub version: String,
}

#[derive(Debug)]
pub struct InstallTool {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Release {
    pub github_token: String,
}

#[derive(Debug)]
pub struct Vendor {
    pub subcommand: VendorCmd,
}

#[derive(Debug)]
pub enum VendorCmd {
    Hash(Hash),
    Normalize(Normalize),
    Generate(Generate),
}

#[derive(Debug)]
pub struct Hash;

#[derive(Debug)]
pub struct Normalize;

#[derive(Debug)]
pub struct Generate;

impl FerrisCi {
    pub const HELP: &'static str = Self::HELP_;

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
