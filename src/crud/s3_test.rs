#[cfg(test)]
mod tests3 {
    use crate::crud::s3::insert_s3_storage;

    #[test]
    fn demo() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }

    #[test]
    fn insert() {
        let bucket = String::from("test");
        let key = String::from("key");
        let res =  insert_s3_storage(&bucket, &key);
        match res {
            Ok(_) => println!("success"),
            Err(_) => panic!("failed"),
        }
    }
}
