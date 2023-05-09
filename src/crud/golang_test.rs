#[cfg(test)]
mod test_go_module {
    use crate::crud::golang::{insert_go_module, get_go_module_id};
    use crate::crud::pool::DbPool;

    #[test]
    fn test_insert_go_module() {
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        let mut conn = pool.get_one_connection();
        let res = insert_go_module(&mut conn, "github", "gin");
        match res {
            Ok(_) => println!("success"),
            Err(_) => panic!("failed to insert"),
        }
    }

    #[test]
    fn get_insert_go_module_id() {
        let url = "postgres://rstore:rstore@localhost:5432/rstore";
        let pool = DbPool::new(url);
        let mut conn = pool.get_one_connection();
        let res = get_go_module_id(&mut conn, "github", "gin");
        match res {
            Ok(id) => println!("Success to get the id {}",id),
            Err(_) => panic!("failed to insert"),
        }
    }
}