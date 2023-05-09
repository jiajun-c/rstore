use axum::{response::IntoResponse, http::StatusCode};

pub enum RError {
    PackageNotFound,
}

impl IntoResponse for RError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            RError::PackageNotFound => "The package not found"
        };
        (StatusCode::NOT_FOUND, body).into_response()
    }
}