use std::fs::OpenOptions;
use std::time::Duration;

use anyhow::bail;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use xshell::Shell;

use crate::flags;

impl flags::InstallTool {
    pub fn run(&self, _sh: &Shell) -> anyhow::Result<()> {
        let pb = ProgressBar::new(500 * 1024 * 1024);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed}] {bytes:>9}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.enable_steady_tick(Duration::from_millis(100));

        let request = ureq::get(&self.url).call()?.into_reader();

        let decode = GzDecoder::new(request);
        let stream = pb.wrap_read(decode);
        let mut archive = tar::Archive::new(stream);
        let mut entries = archive.entries()?;

        let file = &self.name;
        let mut entry = loop {
            let entry = if let Some(entry) = entries.next() {
                entry?
            } else {
                bail!("{file} was not found in archive!");
            };

            if let Some(file_name) = entry.header().path()?.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    if file_name == file {
                        break entry;
                    }
                }
            }
        };

        let path = format!("/bin/{file}");
        let mut options = OpenOptions::new();
        options.create(true);
        options.write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o777);
        }

        let mut file = options.open(&path)?;
        std::io::copy(&mut entry, &mut file)?;

        Ok(())
    }
}
