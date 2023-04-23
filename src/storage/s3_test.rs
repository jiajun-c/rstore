#[cfg(test)]
mod tests3 {

    use aws_sdk_s3::config::{Credentials, Region};

    use crate::{storage::s3::make_bucket};
    #[tokio::test]
    async fn test_make_bucket() {
        let id = "Qpp4IdRalkdoMuC2";
        let secret_key = "cOeTN9UjuP4bTut5U9VHi2vC7fKcVhbK";
        let cred = Credentials::new(id,secret_key, None, None,"loaded-from-custom-env");
        let region = Region::new("cn-north-1");
        let conf_builder = aws_sdk_s3::config::Builder::new().region(region).credentials_provider(cred);
        let conf = conf_builder.build();
        let client = aws_sdk_s3::Client::from_conf(conf);
        let res = make_bucket(&client, "bucket", "cn-north-1").await;
        assert!(res.is_ok(), "{res:?}");
    }
}