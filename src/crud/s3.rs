use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods, BoolExpressionMethods};

use crate::models::storage::NewS3Storage;
use crate::schema::s3storage;

pub fn insert_s3_storage(conn: &mut PgConnection, bucket_name: &str, s3_key: &str) -> Result<usize, diesel::result::Error>{
    let new_s3info = NewS3Storage{bucket_name, s3_key};
    diesel::insert_into(s3storage::table)
            .values(&new_s3info)
            .execute(conn)
}

pub fn delete_s3_storage(conn:&mut PgConnection, bucket_name: &str, s3_key: &str) -> Result<usize, diesel::result::Error>{
    diesel::delete(s3storage::table.filter(s3storage::s3_key.eq(s3_key).and(s3storage::bucket_name.eq(bucket_name)))).execute(conn)
}

// TODO update and check
// Not sure to provide what ?