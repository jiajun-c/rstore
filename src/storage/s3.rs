use s3::bucket_ops::CreateBucketResponse;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{BucketConfiguration, Bucket};
use awsregion::Region;

// pub async ate_bucket(bucket_name: &str) -> Result<CreateBucketResponse, S3Error> {
//     let region =  Region::Custom {
//         region: "eu-central-1".to_owned(),
//         endpoint: "127.0.0.1:9000".to_owned(),
//     };
//     let assess_key = "admin";
//     let secret_key = "admin123";
//     let config = BucketConfiguration::default();
//     let cred = Credentials::new(Some(assess_key), Some(secret_key), None, None, None)?;
//     let create_bucket_respone = Bucket::create(bucket_name, region, cred, config);
//     create_bucket_respone.await
// }

pub struct S3authorization {
    access_key : String,
    secret_key : String,
}

impl S3authorization {
    pub fn new(access_key: String, secret_key: String) -> Self {
        S3authorization{access_key, secret_key}
    }
}

pub async fn create_bucket(bucket_name: &str, endpoint: &str, key: &S3authorization) -> Result<CreateBucketResponse, S3Error> {
    let region_name = "us-east-1".to_string();
    let region = Region::Custom {region: region_name,endpoint:endpoint.to_string()};
    let config = BucketConfiguration::default();
    let cred = Credentials::new(Some(&key.access_key), Some(&key.secret_key), None, None, None).unwrap();
    
    let creat_bucket_response = Bucket::create_with_path_style(bucket_name, region, cred, config).await;
    creat_bucket_response
}
