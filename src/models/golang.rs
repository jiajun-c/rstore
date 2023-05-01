use diesel::prelude::*;

pub struct Goinfo {
    pub id: i32,
    pub package: String,
    pub owner: String,
    pub version: String,
    pub time: String,
    pub domain: String,
    pub bucket_name: String,
    pub path: String,
    pub cloud: bool
}

