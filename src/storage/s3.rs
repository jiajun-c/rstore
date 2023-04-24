use s3::creds::Credentials;
use s3::error::S3Error;
use s3::request::ResponseData;
use s3::Bucket;
use awsregion::Region;

pub struct S3Bucket {
    bucket: Bucket,
}

impl S3Bucket  {
    pub fn new(bucket_name:String, access_key: String, secret_key: String, endpoint: String) -> Self {
        let region_name = "us-east-1".to_string();
        let region = Region::Custom {region: region_name,endpoint};   
        let cred = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap();
        // let resp = Bucket::create_with_path_style(&bucket_name, region, cred, config).await;
        let resp = Bucket::new(&bucket_name, region, cred);
        // let resp: Result<CreateBucketResponse, S3Error> = create_bucket(bucket_name, endpoint, access_key, secret_key).await;
        match resp {
            Ok(resp) => {
                return Self{bucket:resp.with_path_style()};
            },
            Err(_) => {panic!("failed to create the bucket")}
        }
    }

    pub async fn delete(&self) -> Result<u16, S3Error>{
        self.bucket.delete().await
    }

    pub async fn put_object(&self, path: &str, content:&[u8]) -> Result<ResponseData, S3Error>{
        let resp = self.bucket.put_object(path, content).await;
        resp
    }
}