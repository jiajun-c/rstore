use axum::{response::IntoResponse, http::StatusCode};

enum rError {
    PackageNotFound,
}

impl IntoResponse for rError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            rError::PackageNotFound => "The package not found"
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}