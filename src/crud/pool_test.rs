#[cfg(test)]
mod testpool {
    use crate::crud::pool::DbPool;
    #[test]
    fn demo() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }

    #[test]
    fn get_connection_pool() {
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        assert!(pool.pool.max_size() > 0)
    }

    #[test]
    fn get_one_connection_from_pool() {
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        let _ = pool.get_one_connection();
    }
}