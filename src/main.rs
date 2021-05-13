use bytes::BytesMut;
use hex::encode;
use md5::{Digest, Md5};
use rusoto_core::region::Region;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    // Adapted from
    // https://github.com/rusoto/rusoto/blob/master/integration_tests/tests/s3.rs
    let client = S3Client::new(Region::UsEast2);
    let get_req = GetObjectRequest {
        bucket: "encode-test-processing".to_string(),
        key: "caper_out/wgbs/cf5bf980-1a2d-4095-a6af-bd55fb3da2d0/call-make_conf/script"
            .to_string(),
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
        hasher.update(&buffer)
    }
    let final_hash = hasher.finalize();
    println!("{}", encode(final_hash));
}
