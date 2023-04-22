#[cfg(test)]
mod tests3 {
    use crate::crud::s3::insert_s3_storage;
    use crate::crud::pool::DbPool;
    #[test]
    fn demo() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }

    #[test]
    fn insert() {
        let bucket = String::from("test");
        let key = String::from("key");
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        let mut conn = pool.get_one_connection();
        let res =  insert_s3_storage(&mut conn, &bucket, &key);
        match res {
            Ok(_) => println!("success"),
            Err(_) => panic!("failed"),
        }
    }
}
