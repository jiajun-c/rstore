use diesel::RunQueryDsl;

use crate::crud::init::establish_connection;
use crate::models::storage::NewS3Storage;
use crate::schema::s3storage;

pub fn InsertS3Storage(bucket_name: &str, s3_key: &str){
    let connect = &mut establish_connection();
    let new_s3info = NewS3Storage{bucket_name, s3_key};
    diesel::insert_into(s3storage::table)
            .values(&new_s3info)
            .execute(connect)
            .expect("Error saving new s3storage");
}

