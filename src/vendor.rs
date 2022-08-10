use std::path::Path;

use cargo_lock::Lockfile;
use sha2::Digest;
use xshell::{cmd, Shell};

use crate::flags::{Generate, Hash, Normalize, VendorCmd};

pub fn normalize_lockfile(sh: &Shell) -> anyhow::Result<String> {
    let mut lock = Vec::new();
    let lock_file: Lockfile = sh.read_file("Cargo.lock")?.parse()?;
    for package in lock_file.packages {
        if let Some(source) = package.source {
            lock.push((package.name, package.version, source.to_string()))
        }
    }

    lock.sort_by(|(name1, version1, src1), (name2, version2, src2)| {
        name1
            .cmp(name2)
            .then_with(|| version1.cmp(version2))
            .then_with(|| src1.cmp(src2))
    });
    lock.dedup();
    let res = lock
        .into_iter()
        .map(|(name, version, src)| format!("{name}={version}[{src}]\n"))
        .collect();
    Ok(res)
}
impl Normalize {
    fn run(&self, sh: &Shell) -> anyhow::Result<()> {
        let lockfile = normalize_lockfile(sh)?;
        sh.write_file("Cargo.lock.norm", &lockfile)?;
        Ok(())
    }
}

impl Hash {
    pub fn run(&self, sh: &Shell) -> anyhow::Result<()> {
        let lockfile = normalize_lockfile(sh)?;
        let mut hash = sha2::Sha256::new();
        hash.update(lockfile);
        let hash = hash.finalize();
        let hash = base64::encode_config(hash, base64::URL_SAFE);
        println!("{}", hash);
        Ok(())
    }
}

const VENDOR_CFG: &str = "
[source.crates-io]
replace-with = \"vendored-sources\"

[source.vendored-sources]
directory = \"vendor\"
";
impl Generate {
    pub fn run(&self, sh: &Shell) -> anyhow::Result<()> {
        if !sh.path_exists("vendor") {
            cmd!(sh, "cargo vendor").run()?;
        }
        let path = Path::new(".cargo").join("vendor");
        let mut vendor = if sh.current_dir().join(&path).exists() {
            sh.read_file(&path)?
        } else {
            String::new()
        };

        vendor.push_str(VENDOR_CFG);
        sh.write_file(path, vendor)?;
        Ok(())
    }
}

impl VendorCmd {
    pub fn run(&self, sh: &Shell) -> anyhow::Result<()> {
        match self {
            VendorCmd::Hash(cmd) => cmd.run(sh),
            VendorCmd::Generate(cmd) => cmd.run(sh),
            VendorCmd::Normalize(cmd) => cmd.run(sh),
        }
    }
}
