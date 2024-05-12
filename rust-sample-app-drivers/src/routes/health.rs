
use axum::{ http::StatusCode, response::IntoResponse };
use tracing;

pub async fn hc() -> impl IntoResponse {
	tracing::debug!("Access health check endpoint from user!");
	StatusCode::NO_CONTENT
}
