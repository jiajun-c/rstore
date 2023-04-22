
pub mod models;
pub mod crud;
pub mod config;
pub mod schema;
use axum::{
    routing::get,
    Router,
};
use log::{info, warn, error};
use log4rs;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("aabb");
    warn!("ccdd");
    error!("eeff");
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}