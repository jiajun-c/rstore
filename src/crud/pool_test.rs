#[cfg(test)]
mod testpool {
    use crate::crud::pool::DbPool;
    #[test]
    fn demo() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }

    #[test]
    fn get_connection_pool_test() {
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        assert!(pool.pool.max_size() > 0)
    }

}