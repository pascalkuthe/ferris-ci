use std::fs::File;
use std::io::Write;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use xshell::Shell;
use zstd::Encoder;

use crate::{flags, llvm, wine, KnownProgram, git};

impl flags::Archive {
    pub fn run(self, sh: &Shell) -> anyhow::Result<()> {
        let out_file = self.program.archive_name(&self.version, self.debug);
        if !self.upload {
            let mut dst = File::create(sh.current_dir().join(&out_file))?;
            self.run_with_dst(sh, &mut dst)?;
            return Ok(());
        }

        let mut buf: Vec<u8> = Vec::new();
        self.run_with_dst(sh, &mut buf)?;
        if !self.no_save {
            let mut dst = File::create(sh.current_dir().join(&out_file))?;
            dst.write_all(&buf)?;
        }

        crate::s3::upload(&buf, &out_file, self.env_access_key)?;

        Ok(())
    }
    pub fn run_with_dst(&self, sh: &Shell, dst: &mut impl Write) -> anyhow::Result<()> {
        let pb = ProgressBar::new(0);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed}] [{bar:66.cyan/blue}] {bytes:>9}/{total_bytes:9} {bytes_per_second}")
        .unwrap()
        .progress_chars("#>-"))
        ;

        pb.enable_steady_tick(Duration::from_millis(100));

        let mut encoder = self.zstd_encoder(dst)?;
        let mut dst = pb.wrap_write(&mut encoder);
        self.populate_archive(sh, &mut dst, &pb)?;
        encoder.finish()?.flush()?;
        pb.finish_and_clear();
        Ok(())
    }

    fn zstd_encoder<W: Write>(&self, dst: W) -> anyhow::Result<Encoder<'static, W>> {
        let mut encoder = Encoder::new(dst, 22)?;
        encoder.long_distance_matching(true)?;
        encoder.window_log(31)?;
        Ok(encoder)
    }

    fn populate_archive(
        &self,
        sh: &Shell,
        dst: &mut impl Write,
        pb: &ProgressBar,
    ) -> anyhow::Result<()> {
        match self.program {
            KnownProgram::LLVM => llvm::populate_archive(&self.src_dir, sh, dst, pb),
            KnownProgram::Wine => wine::populate_archive(&self.src_dir, sh, dst, pb),
            KnownProgram::Git => git::populate_archive(&self.src_dir, sh, dst, pb),
        }
    }
}
