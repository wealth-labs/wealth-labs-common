use anyhow::Result;
use axum::{
	response::{IntoResponse, Response},
	Json,
};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::fmt::Display;

// ============================================================ //
// Web Json Result
// ============================================================ //

#[derive(Debug)]
pub struct WebJsonResult {
	pub code: u64,
	pub msg: String,
	pub data: Value,
}

impl WebJsonResult {
	pub fn new(code: u64, msg: &str, data: Value) -> Self {
		Self { code, msg: msg.to_owned(), data }
	}

	pub fn ok(data: Value) -> Self {
		Self::new(0, "", data)
	}
	pub fn error(code: u64, msg: &str) -> Self {
		Self::new(code, msg, Value::Null)
	}
}

impl Display for WebJsonResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Code({}), Msg({:?}), data({:?})", self.code, self.msg, self.data)
	}
}

impl IntoResponse for WebJsonResult {
	fn into_response(self) -> Response {
		(
			StatusCode::OK,
			Json(json!({
				"code":self.code,
				"msg":self.msg,
				"data":self.data,
			})),
		)
			.into_response()
	}
}

impl<E> From<E> for WebJsonResult
where
	E: Into<anyhow::Error>,
{
	fn from(value: E) -> Self {
		let err: anyhow::Error = value.into();
		let err_msg = err.to_string();
		if let Ok(result) = err.downcast::<WebJsonResult>() {
			result
		} else {
			tracing::error!("{}", err_msg);
			Self::new(500, "server error", Value::Null)
		}
	}
}

// ============================================================ //
// Web Response
// ============================================================ //

pub type WebResponse = Result<WebJsonResult, WebJsonResult>;
