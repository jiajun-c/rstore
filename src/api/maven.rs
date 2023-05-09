use axum::{extract::{
    Path,
}, response::IntoResponse, http::StatusCode, body::Bytes};
use log::info;
use std::{fs::File, io::{Write, Read}, path::PathBuf};
use crate::api::resp::*;
use axum::Extension;
use std::sync::Arc;
use crate::crud::{pool::DbPool, maven::{get_maven, insert_maven}};
// TODO add the response struction
pub async fn web_get_maven(Path((package_id, group_id,artifact_id,filename)):
    Path<(String, String, String, String)>, state: Extension<Arc<DbPool>>
    ) ->impl IntoResponse {
    let fileinfo = filename.split("/").collect::<Vec<_>>();
    let filename: String;
    let version:String;
    if fileinfo.len() == 1 {
        filename = fileinfo[0].to_string();
        version = String::from("metadata");
    } else {
        version  = fileinfo[0].to_string();
        filename = fileinfo[1].to_string();
    }
    let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> = state.get_one_connection();
    let (_cloud, path) = get_maven(&mut conn,
        &filename,
        &group_id, 
        &artifact_id, 
        &package_id, 
        &version);
    info!("Get file: {}", filename);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            return RError::PackageNotFound.into_response();
        },
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    axum::http::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header("Content-Type", "application/xml")
        .body(axum::body::Body::from(contents))
        .unwrap().into_response()
}

pub async fn web_delete_maven(Path((package_id, group_id,artifact_id,version,filename)):
    Path<(String, String, String, String, String)>, _state: Extension<Arc<DbPool>>) ->String {
    // println!("delete");
    format!("package: {}, group: {}, art: {}, versioN:{}, name:{}",
    package_id, group_id, artifact_id, version, filename)
}

pub async fn web_put_maven(Path((package_id, group_id,artifact_id,filename)):
    Path<(String, String, String, String)>,
    state: Extension<Arc<DbPool>>,
    body: Bytes) ->impl IntoResponse {
    let mut path = format!("/home/bot/{}/{}/{}/",package_id, group_id, artifact_id);
    let fileinfo = filename.split("/").collect::<Vec<_>>();
    let filename: String;
    let version:String;
    if fileinfo.len() == 1 {
        filename = fileinfo[0].to_string();
        version = String::from("metadata");
    } else {
        version  = fileinfo[0].to_string();
        filename = fileinfo[1].to_string();
    }
    path.push_str(&version);
    let _ = std::fs::create_dir_all(&path);
    let filepath = PathBuf::from(&path).join(&filename);
    info!("upload : {}", filename);
    let mut file = File::create(&filepath).unwrap();
    let _ = file.write_all(&body);
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
    (StatusCode::CREATED, "").into_response()
}

// curl --request PUT \
//      --upload-file ./README.md \
//      "http://127.0.0.1:3000/packages/maven/org/example/test/1.0-SNAPSHOT/1.md"