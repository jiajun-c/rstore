use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;

pub struct DbPool {
    pub pool: Pool<ConnectionManager<PgConnection>>
}   
impl DbPool {
    pub fn new(database_url: &str) -> Self{
        let manager: ConnectionManager<_> = ConnectionManager::<PgConnection>::new(database_url);
        let pool =  Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");
        Self {pool}
    }
    pub fn get_one_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get connection from pool.")
    }
}