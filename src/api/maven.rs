use axum::{extract::{
    Path,
    Multipart
}, response::IntoResponse, http::StatusCode};
use log::info;
use std::{fs::File, io::{Write, Read}, path::PathBuf};
use crate::api::resp::*;
use axum::Extension;
use std::sync::Arc;
use crate::crud::{pool::DbPool, maven::{get_maven, insert_maven}};
// TODO add the response struction
pub async fn web_get_maven(Path((package_id, group_id,artifact_id,version,filename)):
    Path<(String, String, String, String, String)>, state: Extension<Arc<DbPool>>) ->impl IntoResponse {
    let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> = state.get_one_connection();
    let (_cloud, path) = get_maven(&mut conn,
        &filename,
        &group_id, 
        &artifact_id, 
        &package_id, 
        &version);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            return rError::PackageNotFound.into_response();
        },
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    axum::http::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .header("Content-Disposition", format!("attachment; filename={}", filename))
        .body(axum::body::Body::from(contents))
        .unwrap().into_response()
    // (StatusCode::OK, String::from("value")).into_response()
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
    let path = format!("/home/bot/{}/{}/{}/{}/",package_id, group_id, artifact_id, version);
    info!("The path: {}", path);
    let _ = std::fs::create_dir_all(&path);
    let filepath = PathBuf::from(&path).join(&filename);
    info!("The path: {}", path);
    let mut file = File::create(&filepath).unwrap();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {} bytes", name, data.len());
        file.write_all(&data).unwrap();
    }
    let mut conn = state.get_one_connection();
    let _res = insert_maven(&mut conn, 
        &filename,
        &group_id, 
        &artifact_id, 
        &package_id,
        &version, 
        "maven", 
        filepath.to_str().unwrap(), 
        &false);
    path
}