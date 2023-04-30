use axum::{response::IntoResponse, http::StatusCode};

pub enum rError {
    PackageNotFound,
}

impl IntoResponse for rError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            rError::PackageNotFound => "The package not found"
        };
        (StatusCode::NOT_FOUND, body).into_response()
    }
}