#[cfg(test)]
mod tests3 {
    use crate::{storage::s3::{S3Bucket}};
    use awsregion::Region;
    use futures::Future;
    use s3::creds::Credentials;
    use s3::{BucketConfiguration, Bucket};

    #[tokio::test]
    async fn test_make_bucket() {
        let access_key = String::from("9wn2DETG5kY3qybY");
        let sercet_key = String::from("8GF5bSzSMNL6jDiKxqIcFWpnaLlZ2Gxk");
        let end_point = String::from("http://localhost:9000");
        let bucket_name = String::from("demp5");
        let region_name = "us-east-1".to_string();
        // let _ = create_bucket(bucket_name, end_point, access_key, sercet_key).await;
        let resp = S3Bucket::new(bucket_name, access_key, sercet_key, end_point);    
    }

    #[tokio::test]
    async fn test_delete_bucket() {
        let access_key = String::from("9wn2DETG5kY3qybY");
        let sercet_key = String::from("8GF5bSzSMNL6jDiKxqIcFWpnaLlZ2Gxk");
        let end_point = String::from("http://localhost:9000");
        let bucket_name = String::from("demp2");
        let buck = S3Bucket::new(bucket_name, access_key, sercet_key, end_point);
        let resp = buck.delete().await;
        match resp {
            Ok(_) => {},
            Err(e) => {println!("{:?}", e);panic!("error in delete")}
        }
    }

    use tokio::runtime::Builder;
    // #[tokio::test]
    #[test]
    fn test_put_object() {
        let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
        let region_name = "us-east-1".to_string();
        let access_key = String::from("9wn2DETG5kY3qybY");
        let sercet_key = String::from("8GF5bSzSMNL6jDiKxqIcFWpnaLlZ2Gxk");
        let end_point = String::from("http://localhost:9000");
        let bucket_name = String::from("demo5");
        let region = Region::Custom {region: region_name,endpoint:end_point};   
        // let config = BucketConfiguration::default();
        let cred = Credentials::new(Some(&access_key), Some(&sercet_key), None, None, None).unwrap();
        // let resp = Bucket::create_with_path_style(&bucket_name, region, cred, config).await;
        let resp = Bucket::new(&bucket_name, region, cred).unwrap().with_path_style();
        let rest = rt.block_on(resp.put_object("/data", "test data".as_bytes()));
        match rest {
            Ok(_) => {},
            Err(e) => {println!("{:?}", e);panic!("error in delete")}
        }
    }
}