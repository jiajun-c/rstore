use diesel::prelude::*;
use crate::schema::{gomodule, goinfo, govesioninfo};

#[derive(Queryable, PartialEq, Debug)]
#[diesel(table_name = gomodule)]
pub struct GoModule {
    pub id: i32,
    pub base: String,
    pub module: String
}

#[derive(Insertable)]
#[diesel(table_name=gomodule)]
pub struct NewGoModule<'a> {
    pub base: &'a str,
    pub module: &'a str
}

#[derive(Insertable, Queryable, PartialEq, Debug)]
#[diesel(table_name = goinfo)]
pub struct GoInfo {
    pub id: i32, // The golang module id
    pub version: String,
    pub time: String,
    pub path: String,
}

#[derive(Insertable, Queryable, PartialEq, Debug)]
#[diesel(table_name = govesioninfo)]
pub struct GoVersionInfo {
    pub mid: i32, // The golang module id
    pub vsc: String,
    pub url: String,
    pub go_ref: String,
    pub go_hash: String,
}

