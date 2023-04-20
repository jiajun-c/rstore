#[cfg(test)]
mod tests3 {
    use crate::crud::s3::InsertS3Storage;

    #[test]
    fn demo() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }

    #[test]
    fn insert() {
        let bucket = String::from("test");
        let key = String::from("key");
        InsertS3Storage(&bucket, &key)
    }
}
