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
	self, sea_query::OnConflict as SeaOrmOnConflict,
	ActiveModelBehavior as SeaOrmActiveModelBehavior, ActiveValue as SeaOrmActiveValue,
	DeriveActiveEnum as SeaOrmDeriveActiveEnum, DeriveEntityModel as OrmDeriveEntityModel,
	DerivePrimaryKey as SeaOrmDerivePrimaryKey, DeriveRelation as SeaOrmDeriveRelation,
	EntityTrait as SeaOrmEntityTrait, EnumIter as SeaOrmEnumIter,
	PrimaryKeyTrait as SeaOrmPrimaryKeyTrait,
};

#[cfg(feature = "web")]
mod web;
#[cfg(feature = "web")]
pub use axum::{
	self,
	extract::Request as AxumRequest,
	middleware::from_fn as axum_middleware_from_fn,
	middleware::Next as AxumMiddlewareNext,
	response::{IntoResponse as AxumIntoResponse, Response as AxumResponse},
	routing::{get as axum_get, post as axum_post},
	Router as AxumRouter,
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
