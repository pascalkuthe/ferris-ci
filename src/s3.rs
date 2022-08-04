use std::cmp::{max, min};
use std::env;
use std::fmt::Write;
use std::sync::{Mutex, RwLock};
use std::thread::available_parallelism;
use std::time::Duration;

use anyhow::{anyhow, bail, Context};
use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use rayon_core::ThreadPoolBuilder;
use rpassword::prompt_password;
use rusty_s3::actions::{
    AbortMultipartUpload, CompleteMultipartUpload, CreateMultipartUpload, PutObject, UploadPart,
};
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use simplerand::rand_range;

use crate::flags::Upload;
use crate::ArchiveFormat;

const BUCKET: &str = "openva";
const REGION: &str = "us-east-1";
const ENDPOINT: &str = "https://fra1.digitaloceanspaces.com";
const TEN_MINUTE: Duration = Duration::from_secs(3600);
const MB: usize = 1024 * 1024;
const CHUNK_SIZE: usize = 8 * MB;
const ETAG: &str = "ETag";
const MAX_RETRY: u32 = 2;

impl Upload {
    pub fn run(&self) -> anyhow::Result<()> {
        let object = self.file.file_name().map(|name| name.to_str());
        let mut object = match object {
            Some(Some(obj)) => obj.to_owned(),
            Some(None) => bail!(
                "s3 upload: filename of {} must be valid utf-8",
                self.file.display()
            ),
            None => bail!("s3 upload: {} is not a file", self.file.display()),
        };

        let mut data = Vec::new();
        match self.compress {
            Some(ArchiveFormat::TarGz) => {
                let encoder = GzEncoder::new(&mut data, Compression::new(7));
                let mut builder = tar::Builder::new(encoder);
                builder
                    .append_path_with_name(&self.file, &object)
                    .with_context(|| format!("failed to read {}", self.file.display()))?;
                builder.into_inner()?.finish()?;
                object = format!("{object}.tar.gz");
            }
            Some(ArchiveFormat::TarZst) => {
                let mut encoder = zstd::Encoder::new(&mut data, 22)?;
                encoder.long_distance_matching(true)?;
                encoder.window_log(31)?;
                let mut builder = tar::Builder::new(encoder);
                builder
                    .append_path_with_name(&self.file, &object)
                    .with_context(|| format!("failed to read {}", self.file.display()))?;
                builder.into_inner()?.finish()?;
                object = format!("{object}.tar.zst");
            }
            None => {
                data = std::fs::read(&self.file)
                    .with_context(|| format!("failed to read {}", self.file.display()))?
            }
        }

        upload(
            &data,
            self.object.as_deref().unwrap_or(&object),
            self.env_access_key,
        )
    }
}

fn get_credentials(env: bool) -> anyhow::Result<Credentials> {
    let (key, secret);
    if env {
        key = env::var("AWS_ACCESS_KEY_ID")?;
        secret = env::var("AWS_SECRET_ACCESS_KEY")?;
    } else {
        key = prompt_password("Enter s3 access-key id")
            .context("failed to read s3 acccess-key id from cli")?;
        secret = prompt_password("Enter s3 access-key secret")
            .context("failed to read s3 access-key secret from cli")?;
    }
    Ok(Credentials::new(key, secret))
}

fn get_bucket() -> Bucket {
    Bucket::new(ENDPOINT.parse().unwrap(), UrlStyle::Path, BUCKET, REGION).unwrap()
}

pub fn upload(data: &[u8], object: &str, env_credentials: bool) -> anyhow::Result<()> {
    let credentials = get_credentials(env_credentials)?;
    let bucket = get_bucket();
    if data.len() < CHUNK_SIZE {
        let mut put_action = PutObject::new(&bucket, Some(&credentials), object);
        put_action.headers_mut().insert("x-amz-acl", "public-read");
        let url = put_action.sign(TEN_MINUTE).to_string();
        ureq::put(&url)
            .set("x-amz-acl", "public-read")
            .send_bytes(data)
            .map_err(s3_err)?;
    } else {
        MultiPartUpload::new(data, object, bucket, credentials, CHUNK_SIZE)?.run()?;
    }
    println!("upload complete");

    Ok(())
}

// pub fn upload_file(path: &Path, env_credentials: bool) -> anyhow::Result<()> {
//     let object = path.file_name().map(|name| name.to_str());
//     let object = match object {
//         Some(Some(obj)) => obj,
//         Some(None) => bail!(
//             "s3 upload: filename of {} must be valid utf-8",
//             path.display()
//         ),
//         None => bail!("s3 upload: {} is not a file", path.display()),
//     };

//     let data = std::fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
//     upload(&data, object, env_credentials)
// }

fn s3_err(err: ureq::Error) -> anyhow::Error {
    if let ureq::Error::Status(status, response) = err {
        let status_text = response.status_text().to_owned();
        let url = response.get_url().to_owned();
        if let Ok(text) = response.into_string() {
            anyhow!("{}: {}\n{}\n{}", status, status_text, text, url,)
        } else {
            anyhow!("{}: {}\n{}", status, status_text, url)
        }
    } else {
        err.into()
    }
}

struct MultiPartUpload<'a> {
    data: &'a [u8],
    object: &'a str,
    bucket: Bucket,
    credentials: Credentials,
    id: String,
    num_parts: u16,
    chunk_size: usize,
}

impl<'a> MultiPartUpload<'a> {
    pub fn new(
        data: &'a [u8],
        object: &'a str,
        bucket: Bucket,
        credentials: Credentials,
        chunk_size: usize,
    ) -> anyhow::Result<Self> {
        let num_parts = Self::partition_data(data.len(), chunk_size)?;
        let res = MultiPartUpload {
            data,
            object,
            bucket,
            credentials,
            id: String::new(),
            num_parts,
            chunk_size,
        };
        Ok(res)
    }

    fn init_connection(&mut self) -> anyhow::Result<()> {
        let mut action =
            CreateMultipartUpload::new(&self.bucket, Some(&self.credentials), self.object);
        action.headers_mut().insert("x-amz-acl", "public-read");
        let url = action.sign(TEN_MINUTE).to_string();
        let resp = ureq::post(&url)
            .set("x-amz-acl", "public-read")
            .call()
            .map_err(s3_err)?
            .into_string()?;

        let multipart = CreateMultipartUpload::parse_response(&resp)?;
        self.id = multipart.upload_id().to_owned();
        Ok(())
    }

    fn partition_data(data_size: usize, chunk_size: usize) -> anyhow::Result<u16> {
        let num_parts = (data_size + chunk_size - 1) / chunk_size;
        if num_parts > 10000 {
            let max_data = HumanBytes((chunk_size * 10000) as u64);
            bail!("can not upload more than 10000 parts ({max_data} GB)")
        }

        Ok(num_parts as u16)
    }

    fn try_upload_chunk(
        &self,
        i: u16,
        etags: &Mutex<Vec<Option<String>>>,
        pb: &ProgressBar,
        data: &mut &[u8],
    ) -> anyhow::Result<()> {
        let part_upload = UploadPart::new(
            &self.bucket,
            Some(&self.credentials),
            self.object,
            (i + 1) as u16,
            &self.id,
        );
        let url = part_upload.sign(TEN_MINUTE);
        let data_len = data.len();
        let reader = pb.wrap_read(data);
        let response = ureq::put(&url.to_string())
            .set("Content-Length", &data_len.to_string())
            .send(reader)
            .map_err(s3_err)?;
        let etag = response.header(ETAG).context("ETag is missting")?;
        let mut dst = etags.lock().unwrap();
        dst[i as usize] = Some(etag.to_owned());
        Ok(())
    }

    fn upload_chunk(
        &self,
        i: u16,
        etags: &Mutex<Vec<Option<String>>>,
        failures: &RwLock<String>,
        pb: &ProgressBar,
    ) {
        let start = (i as usize) * self.chunk_size;
        let end = start + self.chunk_size;
        let end = min(end, self.data.len());
        let data = &self.data[start..end];

        let mut retry = 0;
        loop {
            // already failed somewhere else just exit
            if !failures.read().unwrap().is_empty() {
                return;
            }
            let mut send_buf = data;
            match self.try_upload_chunk(i, etags, pb, &mut send_buf) {
                Ok(_) => return,
                Err(_) if retry <= MAX_RETRY => {
                    let off = (data.len() - send_buf.len()) as u64;
                    pb.set_position(pb.position() - off);
                    let max_backoff = min((1u32 << retry) * 1000, 20000);
                    retry += 1;
                    let min_backoff = 100;
                    let backoff = rand_range(min_backoff, max_backoff);
                    let backoff = Duration::from_millis(backoff as u64);
                    std::thread::sleep(backoff)
                }
                Err(err) => {
                    let mut failures = failures.write().unwrap();
                    failures.push('\n');
                    write!(failures, "{}", err).unwrap();
                    return;
                }
            }
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.init_connection().context("multipart init failed")?;

        let num_threads = max(available_parallelism().unwrap().into(), 10);
        let num_threads = min(num_threads, self.num_parts as usize);
        let pool = ThreadPoolBuilder::default()
            .num_threads(num_threads)
            .build()?;

        let etags = Mutex::new(vec![None; self.num_parts as usize]);
        let failures = RwLock::new(String::new());

        let pb = ProgressBar::new(self.data.len() as u64);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed}] [{bar:66.cyan/blue}] {bytes:>9}/{total_bytes:9} ({eta})")
        .unwrap()
        .progress_chars("#>-"))
        ;

        pb.enable_steady_tick(Duration::from_millis(100));

        pool.in_place_scope(|scope| {
            let etags = &etags;
            let failures = &failures;
            let connection = &self;
            let pb = &pb;
            for i in 0..self.num_parts {
                scope.spawn(move |_| connection.upload_chunk(i, etags, failures, pb))
            }
        });

        let failures = failures.into_inner().unwrap();
        if !failures.is_empty() {
            let abort_action = AbortMultipartUpload::new(
                &self.bucket,
                Some(&self.credentials),
                self.object,
                &self.id,
            );
            let url = abort_action.sign(TEN_MINUTE).to_string();
            #[allow(unused_must_use)] // we already have an error here better report that
            {
                ureq::post(&url).call();
            }
            bail!("upload failed: {failures}");
        }
        let etags = etags.into_inner().unwrap();
        let complete_action = CompleteMultipartUpload::new(
            &self.bucket,
            Some(&self.credentials),
            self.object,
            &self.id,
            etags.iter().map(|tag| tag.as_deref().unwrap()),
        );
        let url = complete_action.sign(TEN_MINUTE).to_string();
        let body = complete_action.body();
        ureq::post(&url)
            .send_bytes(body.as_bytes())
            .map_err(s3_err)?;
        pb.finish_and_clear();
        Ok(())
    }
}
