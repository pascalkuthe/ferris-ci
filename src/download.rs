use std::fs::File;
use std::io::{self, Read};
use std::time::Duration;

use anyhow::bail;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use xshell::Shell;
use zstd::Decoder;

use crate::flags;

const BASE_URL: &str = "https://openva.fra1.cdn.digitaloceanspaces.com";

impl flags::Download {
    pub fn run(&self, sh: &Shell) -> anyhow::Result<()> {
        let pb = ProgressBar::new(500 * 1024 * 1024);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed}] {bytes:>9}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.enable_steady_tick(Duration::from_millis(100));

        let stream = self.request_file()?;

        if !self.decompress {
            let mut stream = pb.wrap_read(stream);

            let dst = sh.current_dir().join(&self.file);
            let mut dst = File::create(dst)?;
            io::copy(&mut stream, &mut dst)?;
            return Ok(());
        }

        match self.file.rsplit_once('.').map(|(start,ext)| (start.rsplit_once('.').map(|(_, ext)| ext), ext)) {
            Some((Some("tar"), "zst" | "zstd")) => {
                let mut decoder = Decoder::new(stream)?;
                decoder.window_log_max(31)?;
                let stream = pb.wrap_read(decoder);
                tar::Archive::new(stream).unpack(sh.current_dir())?;
            }

            Some((Some("tar"), "gz")) => {
                let decoder = GzDecoder::new(stream);
                let stream = pb.wrap_read(decoder);
                tar::Archive::new(stream).unpack(sh.current_dir())?;
            }

            Some((Some(prefix), ext)) => bail!("unkown archive type {prefix}.{ext}, ferris-ci currently supports 'tar.zst', tar.zstd' and 'tar.gz'"),
            Some((None, ext)) => bail!("unkown archive type {ext}, ferris-ci currently supports 'tar.zst', tar.zstd' and 'tar.gz'"),
            None => bail!("archive type could not be determined, because {} does not contain a file extension", &self.file),
        }

        pb.finish_and_clear();
        Ok(())
    }
    pub fn request_file(&self) -> anyhow::Result<Box<dyn Read + Send + Sync + 'static>> {
        let base_url = self.base_url.as_deref().unwrap_or(BASE_URL);
        let url = format!("{base_url}/{}", &self.file);
        let resp = ureq::get(&url).call()?.into_reader();
        Ok(resp)
    }
}
