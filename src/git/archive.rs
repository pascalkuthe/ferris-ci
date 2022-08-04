use std::fs;
use std::io::Write;
use std::path::PathBuf;

use indicatif::ProgressBar;
use tar::{EntryType, Header};
use xshell::Shell;

use crate::git::BUILD_DIR;

pub fn populate_archive(
    src_dir: &Option<PathBuf>,
    sh: &Shell,
    dst: &mut impl Write,
    pb: &ProgressBar,
) -> anyhow::Result<()> {
    let src_dir = src_dir
        .clone()
        .unwrap_or_else(|| sh.current_dir().join(BUILD_DIR));

    let mut tar_builder = tar::Builder::new(dst);
    pb.set_length(5000);
    tar_builder.follow_symlinks(false);
    tar_builder.append_dir_all("GIT/bin", src_dir.join("bin"))?;
    tar_builder.append_dir("GIT/libexec", src_dir.join("libexec"))?;
    let libexec = src_dir.join("libexec").join("git-core");
    tar_builder.append_dir("GIT/libexec/git-core", &libexec)?;
    for file in fs::read_dir(libexec)? {
        let file = file?;
        if let Some(name) = file.path().file_name().and_then(|name| name.to_str()) {
            let hardlink = match name {
                "git-daemon"
                | "git-remote-http"
                | "git-svn"
                | "git-imap-send"
                | "git-http-backend"
                | "git-web--browse"
                | "git-send-email"
                | "git-remote-ftp"
                | "git-remote-ftps"
                | "git-remote-https"
                | "git-instaweb"
                | "git-add--interactive"
                | "git-bisect"
                | "git-cvsexportcommit"
                | "git-cvsimpot"
                | "git-cvsserver"
                | "git-merge-octopus"
                | "git-merge-one-file"
                | "git-merge-resolve"
                | "git-mergetool"
                | "git-p4"
                | "mergetools" => continue,
                "git-sh-setup" | "git-sh-i18n--envsubst" | "git-sh-i18n" | "git-shell" => false,
                _ => true,
            };

            let dst = format!("GIT/libexec/git-core/{name}");

            if hardlink {
                let mut header = Header::new_gnu();
                header.set_entry_type(EntryType::hard_link());
                header.set_size(0);
                tar_builder.append_link(&mut header, dst, "GIT/bin/git")?;
            } else {
                tar_builder.append_file(dst, &mut fs::File::open(file.path())?)?;
            }
        }
    }

    Ok(())
}
