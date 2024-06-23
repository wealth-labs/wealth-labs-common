use super::types::WebJsonResult;
use axum::{
	extract::Request,
	middleware::Next,
	response::{IntoResponse, Response},
};
use tokio::time::Instant;
use tracing::{info_span, Instrument};

pub async fn web(request: Request, next: Next) -> Response {
	let trace_id = request
		.headers()
		.get("X-TRACE-ID")
		.map(|v| v.to_str().map(|v| v.to_owned()).unwrap_or(crate::uuid()))
		.unwrap_or(crate::uuid());
	let _span = info_span!("web", traceid = trace_id);
	let _enter = _span.enter();
	let response = next.run(request).in_current_span().await;
	drop(_enter);
	response
}

pub async fn request_time(request: Request, next: Next) -> Response {
	let method = request.method().to_string().to_lowercase();
	let uri = request.uri().to_string();
	let instant = Instant::now();
	let response = next.run(request).in_current_span().await;
	tracing::info!("request time: {} {} - {:?}", method, uri, instant.elapsed());
	response
}

// ============================================================ //
// 404
// ============================================================ //

pub async fn handler_404() -> Response {
	WebJsonResult::error(404, "resource not found").into_response()
}
