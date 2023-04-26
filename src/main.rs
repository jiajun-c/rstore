use axum::Router;
use axum::extract::Path;
use axum::{
    routing::{get}
};
pub mod models;
pub mod crud;
pub mod config;
pub mod schema;
pub mod storage;
// cargo run --example minio

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route("/", get(root))
    .route("/repo/:user/:repo", get(repo_info));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hello world"
}


async fn repo_info(Path((user_name, repo_name)): Path<(String, String)>) -> String {
    format!(
        "Repo: user_name: {} repo_name: {}",
        user_name, repo_name
    )
}