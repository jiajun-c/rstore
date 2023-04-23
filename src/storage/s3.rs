use s3::creds::Credentials;
use s3::{error::S3Error, Bucket};
use s3::Region;
pub fn create_bucket(bucket_name: &str) {
    let bucket = Bucket::new(
        "test-rust-s3",
        Region::Custom {
             region: "eu-central-1".to_owned(),
             endpoint: "http://localhost:9000".to_owned(),
        },
        Credentials::default().unwrap()
    );
}