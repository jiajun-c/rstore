use std::{sync::Arc, path::Path};

use api::maven::*;
use axum::{
    Router,
    Extension,
    routing::*,
    extract::DefaultBodyLimit,
};

use crud::pool::DbPool;
use log::info;

use crate::rconfig::setting::Settings;
pub mod models;
pub mod crud;
pub mod rconfig;
pub mod api;
pub mod schema;
pub mod storage;
// cargo run --example minio

#[tokio::main]
async fn main() {
    // Init the log
    log4rs::init_file("/home/bot/workspace/rstore/log4rs.yaml", Default::default()).unwrap();
    info!("rstore started");

    // Init the database
    let pool = DbPool::new("postgres://rstore:rstore@localhost:5432/rstore");

    // Init the config settings
    let setting = Settings::new("/home/bot/workspace/rstore/rstore.toml".to_string()).unwrap();
    info!("Listen on the port: {}", setting.server.port);
    info!("database url: {}", setting.db.url);
    let pool = Arc::new(pool);

    let maven_router = Router::new()
        .route("/:packageId/:groupId/:artifactId/:version/:filename", 
            delete(web_delete_maven)
            .put(web_put_maven)
            .get(web_get_maven))
            .layer(DefaultBodyLimit::disable());
    // Print out our settings (as a HashMap)
    // build our application with a single route
    let app = Router::new().nest("/packages/maven/", maven_router)
        .layer(Extension(pool));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

