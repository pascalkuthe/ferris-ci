use xshell::{cmd, Shell};

use crate::flags::Build;
use crate::wine::{BUILD_DIR, SRC_DIR};

impl Build {
    pub fn run_wine(&self, sh: &Shell) -> anyhow::Result<()> {
        const URL: &str = "https://gitlab.winehq.org/wine/wine.git";
        let tag = format!("wine-{}", self.version);
        self.get_src(sh, URL, SRC_DIR, &tag)?;

        let build_dir = format!("{SRC_DIR}/{BUILD_DIR}");
        std::fs::create_dir(&build_dir)?;
        let _guard = sh.push_dir(format!("{SRC_DIR}/{BUILD_DIR}"));

        self.configure_wine(sh)?;
        self.build_wine(sh)?;
        Ok(())
    }

    fn configure_wine(&self, sh: &Shell) -> anyhow::Result<()> {
        let configure = sh
            .current_dir()
            .parent()
            .unwrap()
            .join("configure");

        let install_dir = sh
            .current_dir()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(BUILD_DIR);

        let mut cmd = cmd!(
            sh,
            "{configure} --enable-win64 --disable-win16 --disable-tests --without-x --without-freetype --prefix {install_dir} --libdir={install_dir}/lib64"
        );
        cmd = self.add_cc_flags(cmd);
        cmd.run()?;

        Ok(())
    }

    fn build_wine(&self, sh: &Shell) -> anyhow::Result<()> {
        let mut cmd = cmd!(sh, "make install");
        cmd = self.add_parallel_flag(cmd);
        cmd = self.add_cc_flags(cmd);
        cmd.run()?;

        Ok(())
    }
}
