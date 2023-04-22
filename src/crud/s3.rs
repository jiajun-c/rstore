use diesel::{RunQueryDsl, PgConnection};

use crate::models::storage::NewS3Storage;
use crate::schema::s3storage;

pub fn insert_s3_storage(conn: &mut PgConnection, bucket_name: &str, s3_key: &str) -> Result<usize, diesel::result::Error>{
    let new_s3info = NewS3Storage{bucket_name, s3_key};
    diesel::insert_into(s3storage::table)
            .values(&new_s3info)
            .execute(conn)
}

// pub fn delete_s3_storage(bucket_name: &str, s3_key: &str) {
    
// }