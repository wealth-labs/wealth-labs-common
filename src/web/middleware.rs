use super::types::WebJsonResult;
use axum::{
	extract::Request,
	middleware::Next,
	response::{IntoResponse, Response},
};
use tokio::time::Instant;

pub async fn request_time(request: Request, next: Next) -> Response {
	let method = request.method().to_string().to_lowercase();
	let uri = request.uri().to_string();
	let instant = Instant::now();
	let response = next.run(request).await;
	tracing::info!("method({}), uri({}), time({:?})", method, uri, instant.elapsed());
	response
}

// ============================================================ //
// 404
// ============================================================ //

pub async fn handler_404() -> Response {
	WebJsonResult::error(404, "resource not found").into_response()
}
