use super::types::WebJsonResult;
use axum::{
	extract::Request,
	middleware::Next,
	response::{IntoResponse, Response},
};
use tokio::time::Instant;
use tracing::info_span;

// ============================================================ //
// Trace ID
// ============================================================ //

pub async fn trace_id(request: Request, next: Next) -> Response {
	let trace_id = request
		.headers()
		.get("X-TRACE-ID")
		.map(|v| v.to_str().map(|v| v.to_owned()).unwrap_or(crate::uuid()))
		.unwrap_or(crate::uuid());
	let _span = info_span!("traceid", value = trace_id);
	let _enter = _span.enter();
	let response = next.run(request).await;
	drop(_enter);
	response
}

// ============================================================ //
// Log
// ============================================================ //

pub async fn logger(request: Request, next: Next) -> Response {
	let method = request.method().to_string();
	let uri = request.uri().to_string();
	let instant = Instant::now();
	let response = next.run(request).await;
	tracing::info!("http request: {} {} - {:?}", method, uri, instant.elapsed());
	response
}

// ============================================================ //
// 404
// ============================================================ //

pub async fn handler_404() -> Response {
	WebJsonResult::error(404, "resource not found").into_response()
}
