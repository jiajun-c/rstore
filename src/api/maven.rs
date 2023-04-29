use axum::{extract::{
    Path,
    Multipart
}, response::IntoResponse};
use std::{fs::File, io::Write};
use axum::Extension;
use std::sync::Arc;
use crate::crud::{pool::DbPool, maven::{get_maven, insert_maven}};
// TODO add the response struction
pub async fn web_get_maven(Path((package_id, group_id,artifact_id,version,filename)):
    Path<(String, String, String, String, String)>, state: Extension<Arc<DbPool>>) ->String {
    let mut conn = state.get_one_connection();
    let (_cloud, path) = get_maven(&mut conn,
        &filename,
        &group_id, 
        &artifact_id, 
        &package_id, 
        &version);
    path
}

pub async fn web_delete_maven(Path((package_id, group_id,artifact_id,version,filename)):
    Path<(String, String, String, String, String)>, _state: Extension<Arc<DbPool>>) ->String {
    // println!("delete");
    format!("package: {}, group: {}, art: {}, versioN:{}, name:{}",
    package_id, group_id, artifact_id, version, filename)
}

pub async fn web_put_maven(Path((package_id, group_id,artifact_id,version,filename)):
    Path<(String, String, String, String, String)>,  state: Extension<Arc<DbPool>>,mut multipart: Multipart) ->String {
    // println!("delete");
    let mut base = String::from("/home/bot/workspace/rstore/");
    base.push_str(&filename);
    let mut file = File::create(base).unwrap();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {} bytes", name, data.len());
        file.write_all(&data).unwrap();
    }
    let mut conn = state.get_one_connection();
    let path = format!("package: {}, group: {}, art: {}, versioN:{}, name:{}",package_id, group_id, artifact_id, version, filename);
    let _res = insert_maven(&mut conn, 
        &filename,
        &group_id, 
        &artifact_id, 
        &package_id,
        &version, 
        "maven", 
        &path, 
        &false);
    path
}