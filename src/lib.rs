mod app;
pub use app::{init as app_init, ins as app_ins, App};

mod util;
pub use util::uuid;

mod config;
pub use config::init as config_init;

mod logger;
pub use logger::{init as logger_init, Config as LoggerConfig};

#[cfg(feature = "database")]
mod database;
#[cfg(feature = "database")]
pub use database::{init as database_init, ins as database_ins, Config as DatabaseConfig};
#[cfg(feature = "database")]
pub use sea_orm::{
	self, sea_query::Expr, sea_query::OnConflict, sea_query::StringLen, ActiveModelBehavior,
	ActiveValue, ColumnTrait as _, DeriveActiveEnum, DeriveEntityModel, DerivePrimaryKey,
	DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, QueryFilter as _,
};

#[cfg(feature = "web")]
mod web;
#[cfg(feature = "web")]
pub use axum::{
	self,
	extract::Request,
	middleware::from_fn,
	middleware::Next,
	response::{IntoResponse, Response},
	routing::{get, post},
	Router,
};
#[cfg(feature = "web")]
pub use web::{init as web_init, types::WebJsonResult, types::WebResponse, Config as WebConfig};

pub use anyhow::{self as _anyhow, anyhow, bail, ensure, Result};
pub use async_trait::async_trait;
pub use chrono::{self, DateTime, Duration, FixedOffset, Utc};
pub use once_cell::{self, sync::OnceCell};
pub use reqwest;
pub use rust_decimal::{self, Decimal};
pub use serde::{self, Deserialize, Serialize};
pub use serde_json::{self, Map as JsonMap, Value as Json};
pub use serde_with::{self, serde_as};
pub use std::{result::Result as StdResult, str::FromStr, time::Duration as StdDuration};
pub use tokio::{
	self, spawn,
	sync::{
		mpsc::{channel as mpsc_channel, Receiver as MpscReceiver, Sender as MpscSender},
		Mutex,
	},
	time::sleep,
};
pub use tracing::{self, debug, error, info, instrument, warn};
