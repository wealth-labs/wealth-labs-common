pub mod middleware;
pub mod types;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub name: String,
	pub listen: String,
	pub show_log: bool,
}

pub async fn init(
	config: &Config,
	register_router: fn(app: axum::Router) -> axum::Router,
) -> Result<()> {
	let app_stop_notice = crate::app::ins().create_app_stop_notice().await;
	let (app_waiting_send, app_waiting_recv) = channel::<Result<()>>(1);
	crate::app::ins().add_app_waiting(&config.name, app_waiting_recv).await;
	let listen = config.listen.to_owned();
	let show_log = config.show_log;
	tokio::spawn(async move {
		let result = run(listen, show_log, register_router, app_stop_notice).await;
		app_waiting_send.send(result).await.ok();
	});
	tracing::info!("web({}) running .....", config.name);
	Ok(())
}

async fn run(
	listen: String,
	show_log: bool,
	register_router: fn(app: axum::Router) -> axum::Router,
	mut notice: Receiver<()>,
) -> Result<()> {
	let mut app = axum::Router::new();
	app = app.layer(axum::middleware::from_fn(middleware::trace_id));
	if show_log {
		app = app.layer(axum::middleware::from_fn(middleware::logger));
	}
	app = app.fallback(middleware::handler_404);
	app = register_router(app);
	let listener = tokio::net::TcpListener::bind(listen).await?;
	axum::serve(listener, app)
		.with_graceful_shutdown(async move {
			notice.recv().await;
		})
		.await?;
	Ok(())
}
