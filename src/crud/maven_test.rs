#[cfg(test)]
mod test_maven {
    use crate::crud::maven::{insert_maven, delete_maven, get_maven};
    use crate::crud::pool::DbPool;
    #[test]
    fn demo() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }

    #[test]
    fn test_maven_insert() {
        let bucket = String::from("test");
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        let mut conn = pool.get_one_connection();
        let res =  insert_maven(&mut conn, "aa", "bb", "cc", "dd", "ee", &bucket, "zz", &false);
        match res {
            Ok(_) => println!("success"),
            Err(_) => panic!("failed"),
        }
    }

    #[test]
    fn test_maven_delete() {
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        let mut conn = pool.get_one_connection();
        let res =  delete_maven(&mut conn, "aa", "bb", "cc", "dd", "ee");
        match res {
            Ok(_) => println!("success"),
            Err(_) => panic!("failed"),
        }
    }

    #[test]
    fn test_maven_get() {
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        let mut conn = pool.get_one_connection();
        let res =  get_maven(&mut conn, "aa", "bb", "cc", "dd", "ee");
        assert_eq!(res.1, "zz");
    }
}