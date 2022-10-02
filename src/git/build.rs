use xshell::{cmd, Shell};

use crate::flags::Build;
use crate::git::{BUILD_DIR, SRC_DIR};
const CONFIG_MAK: &str = "NO_GETTEXT=YesPlease
NO_SVN_TESTS=YesPlease
NO_SYS_POLL_H=1
CFLAGS=-Os -m64
LDFLAGS=-m64
ICONV_OMITS_BOM=Yes
NO_PYTHON=YesPlease
NO_TCLTK=YesPlease
NO_EXPAT=YesPlease
NO_CURL=YesPlease
NO_GITWEB=YesPlease
NO_PERL=YesPlease
";
impl Build {
    pub fn run_git(&self, sh: &Shell) -> anyhow::Result<()> {
        const URL: &str = "https://github.com/git/git.git";
        let tag = format!("v{}", self.version);
        self.get_src(sh, URL, SRC_DIR, &tag)?;

        let _guard = sh.push_dir(SRC_DIR);
        sh.write_file("config.mak", CONFIG_MAK)?;

        sh.remove_path(BUILD_DIR)?;
        self.make_git(sh, false)?;
        self.make_git(sh, true)?;
        Ok(())
    }

    fn make_git(&self, sh: &Shell, install: bool) -> anyhow::Result<()> {
        let install_dir = sh.current_dir().parent().unwrap().join(BUILD_DIR);
        let mut cmd = cmd!(
            sh,
            "make prefix=/. lib=lib64 DESTDIR={install_dir} NO_INSTALL_HARDLINKS=1"
        );
        if install {
            cmd = cmd.arg("install");
        }
        cmd.run()?;
        Ok(())
    }
}
