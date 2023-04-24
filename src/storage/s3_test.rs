#[cfg(test)]
mod tests3 {
    use crate::{storage::s3::{create_bucket, S3authorization}};
    #[tokio::test]
    async fn test_make_bucket() {
        let access_key = String::from("uX1tWH3BI4y3hGjN");
        let sercet_key = String::from("46RKz12R0rBHam8yEinBKNtpXn1j94eB");
        let author = S3authorization::new(access_key, sercet_key);
        let end_point = "http://localhost:9000";
        let bucket_name = "demo";
        let ans = create_bucket(bucket_name, end_point, &author).await;
        match ans {
            Err(err) => {println!("{:?}", err); panic!("err");},
            Ok(_) => println!("ok"),
        }
    }
}