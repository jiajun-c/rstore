use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: ConnectionManager<_> = ConnectionManager::<PgConnection>::new(url);
    let pool =  Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");
    pool
}

#[cfg(test)]
mod test {
    use super::*;
    

}