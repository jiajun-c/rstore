use diesel::prelude::*;

use crate::schema::s3storage;

#[derive(Queryable)]
pub struct S3Storage {
    pub id: i32,
    pub bucket_name: String,
    pub s3_key: String,
}

#[allow(dead_code)]
#[derive(Insertable)]
#[diesel(table_name = s3storage)]
pub struct NewS3Storage<'a> {
    pub bucket_name: &'a str,
    pub s3_key: &'a str,
}

#[allow(dead_code)]
pub struct LocalStorage {
    pub path: String,
}