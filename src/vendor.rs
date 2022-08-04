use std::cmp::Ordering;
use std::path::Path;

use sha2::Digest;
use xshell::{cmd, Shell};

use crate::flags::{Generate, Hash, Normalize, VendorCmd};

pub fn normalize_lockfile(sh: &Shell) -> anyhow::Result<String> {
    let mut lock = Vec::new();
    let lock_file = sh.read_file("Cargo.lock")?;
    // Only hash the dependencies not the local crates
    for package in lock_file.split("\n[[package]]") {
        if !package.contains("source") {
            continue;
        }
        let mut name = None;
        let mut version = None;
        for line in package.split('\n') {
            if line.contains("name ") {
                name = line.split('\"').nth(1)
            }
            if line.contains("version ") {
                version = line.split('\"').nth(1)
            }
        }

        let (name, version) = (name.unwrap(), version.unwrap());
        lock.push((name, version))
    }
    lock.sort_by(
        |(name1, version1), (name2, version2)| match name1.cmp(name2) {
            Ordering::Equal => version1.cmp(version2),
            res => res,
        },
    );
    lock.dedup();
    let res = format!("{lock:#?}");
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
