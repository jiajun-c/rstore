#[cfg(test)]
mod tests3 {

    use crate::{storage::s3::make_bucket};
    #[tokio::test]
    async fn test_make_bucket() {
        let id = "Qpp4IdRalkdoMuC2";
        let secret_key = "cOeTN9UjuP4bTut5U9VHi2vC7fKcVhbK";
        let endpoint = "http://211.67.25.90:9010/";
        
        let res = make_bucket(&client, "bucket", "cn-north-1").await;
        assert!(res.is_ok(), "{res:?}");
    }
}