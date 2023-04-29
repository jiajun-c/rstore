use std::sync::Arc;

use axum::{Router, Extension, http::Request};
use api::maven::*;
use axum::{
    routing::*,
    extract::DefaultBodyLimit,
    extract::{Path, State}
};
use crud::pool::DbPool;
use log::info;
pub mod models;
pub mod crud;
pub mod config;
pub mod api;
pub mod schema;
pub mod storage;
// cargo run --example minio

#[tokio::main]
async fn main() {
    log4rs::init_file("/home/bot/workspace/rstore/log4rs.yaml", Default::default()).unwrap();
    info!("rstore started");
    let pool = DbPool::new("postgres://rstore:rstore@localhost:5432/rstore");

    let pool = Arc::new(pool);

    let maven_router = Router::new()
        .route("/:packageId/:groupId/:artifactId/:version/:filename", 
        get(get_maven)
            .delete(delete_maven)
            .put(put_maven))
            .layer(DefaultBodyLimit::disable());

    // build our application with a single route
    let app = Router::new().nest("/packages/maven/", maven_router)
        .layer(Extension(pool));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

