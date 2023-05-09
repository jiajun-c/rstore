use diesel::prelude::*;

pub struct GoModule {
    pub id: i32,
    pub base: String,
    pub module: String
}

pub struct GoInfo {
    pub mid: i32, // The golang module id
    pub version: i32,
    pub time: String,
    pub path: String,
}

pub struct GoVersionInfo {
    pub mid: i32, // The golang module id
    pub vsc: String,
    pub url: String,
    pub go_ref: String,
    pub go_hash: String,
}

