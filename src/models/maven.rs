use diesel::prelude::*;
use crate::schema::mavens;
#[derive(Queryable)]
pub struct Maven {
    pub id: i32,
    pub name: String,
    pub group_id: String,
    pub artifact_id: String,
    pub packaging: String,
    pub version: String,
    pub bucket_name:String,
    pub path: String,
    pub cloud: bool
}

#[derive(Insertable)]
#[diesel(table_name = mavens)]
pub struct NewMaven<'a> {
    pub name: &'a str,
    pub group_id: &'a str,
    pub artifact_id: &'a str,
    pub packaging: &'a str,
    pub version: &'a str,
    pub bucket_name: &'a str,
    pub path: &'a str,
    pub cloud: &'a bool
}