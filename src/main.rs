use bytes::BytesMut;
use hex;
use md5::{Digest, Md5};
use rusoto_core::region::Region;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use structopt::StructOpt;
use tokio::io::AsyncReadExt;
use url::Url;

#[derive(Debug, StructOpt)]
#[structopt(name = "s3-md5", about = "Util to calculate md5sums of s3 objects")]
struct Args {
    /// S3 URI to compute md5sum, should take form of s3://foo/bar/baz.qux
    #[structopt()]
    s3_uri: String,
}

struct S3Uri {
    bucket: String,
    key: String,
}

impl S3Uri {
    fn from_string(uri: &str) -> Self {
        let parsed = Url::parse(uri).unwrap();
        return Self {
            bucket: parsed.host_str().unwrap().to_string(),
            key: parsed.path().trim_start_matches("/").to_string(),
        };
    }
}

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let s3_uri = S3Uri::from_string(&args.s3_uri);
    // Adapted from
    // https://github.com/rusoto/rusoto/blob/master/integration_tests/tests/s3.rs
    let client = S3Client::new(Region::UsEast2);
    let get_req = GetObjectRequest {
        bucket: s3_uri.bucket,
        key: s3_uri.key,
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .await
        .expect("Couldn't GET object");

    let stream = result.body.unwrap();
    let mut hasher = Md5::new();
    let mut body = stream.into_async_read();
    let mut buffer = BytesMut::with_capacity(8192);
    loop {
        let result = body.read_buf(&mut buffer).await.unwrap();
        match result {
            0 => break,
            _ => {}
        }
        hasher.update(&buffer[..result]);
        // We never read uninitialized data from the buffer, so this is OK. `read_buf`
        // will set the bytes and we only pass read bytes in the slice to the hasher.
        unsafe {
            buffer.set_len(0);
        };
    }
    let final_hash = hasher.finalize();
    println!("{}", hex::encode(final_hash));
}
