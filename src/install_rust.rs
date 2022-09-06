use std::fs::OpenOptions;

use xshell::{cmd, Shell};

use crate::{flags, TARGET};

const FILE: &str = "rustup-init";

impl flags::InstallRust {
    pub fn run(&self, sh: &Shell) -> anyhow::Result<()> {
        let version = &self.version;
        let url = format!("https://static.rust-lang.org/rustup/archive/1.24.3/{TARGET}/{FILE}");
        let mut request = ureq::get(&url).call()?.into_reader();
        let path = sh.current_dir().join(FILE);
        let mut options = OpenOptions::new();
        options.create(true);
        options.write(true);

        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o777);
        }

        let mut file = options.open(&path)?;
        std::io::copy(&mut request, &mut file)?;
        drop(file);
        cmd!(sh, "{path}  -y --no-modify-path --profile minimal --default-toolchain {version} --default-host {TARGET}").run()?;
        sh.remove_path(&path)?;

        #[cfg(windows)]
        cmd!(sh, "powershell -command RefreshEnv").run()?;
        #[cfg(unix)]
        {
            let rustup_home = std::env::var_os("RUSTUP_HOME").unwrap();
            let cargo_home = std::env::var_os("CARGO_HOME").unwrap();
            cmd!(sh, "chmod -R a+w {rustup_home} {cargo_home}").run()?;
        }

        Ok(())
    }
}
