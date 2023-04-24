pub mod models;
pub mod crud;
pub mod config;
pub mod schema;
pub mod storage;
// cargo run --example minio

use awsregion::Region;
use s3::creds::Credentials;
use s3::{Bucket, BucketConfiguration};

#[tokio::main]
async fn main() {
    let bucket_name = "gentoo";
    let region_name = "us-east-1".to_string();
    let endpoint = "http://localhost:9000".to_string();
    let region =  Region::Custom { region: region_name, endpoint };
    let config = BucketConfiguration::default();
    let assess_key = "admin";
    let secret_key = "admin123";
    let cred = Credentials::new(Some(assess_key), Some(secret_key), None, None, None).unwrap();
    
    // // Async variant with `tokio` or `async-std` features
    let create_bucket_response = Bucket::create_with_path_style(bucket_name, region, cred, config).await;
    match create_bucket_response {
        Ok(_) => {}
        Err(err) => {println!("{:?}", err)}
    }
}