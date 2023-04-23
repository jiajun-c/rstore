use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::{operation::create_bucket::{CreateBucketError, CreateBucketOutput}, types::{BucketLocationConstraint, CreateBucketConfiguration}};

pub async fn make_bucket(client: &Client, bucket: &str, region: &str) -> Result<CreateBucketOutput, SdkError<CreateBucketError>>{
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket)
        .send().await
}