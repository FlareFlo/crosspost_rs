use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude::UserId;
use sqlx::sqlite::SqlitePoolOptions;
use time::{OffsetDateTime, UtcOffset};
use tokio::sync::RwLock;

use crate::commands::{channel_status, diagnose, disable_crosspost, enable_crosspost, ping, register, register_global};
use crate::db::cross_db::CrossDb;
use crate::handler::event_listener;
use crate::info::Sys;

mod handler;
mod commands;
mod db;
mod info;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
pub struct Data {
	db: CrossDb,
	start_date: i64,
	sys: Arc<RwLock<Sys>>,
}

#[tokio::main]
async fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	// Fun with the path, as the path fn properly escapes paths for windows or linux
	let env = std::env::var("DATABASE_URL").unwrap().clone();
	let path = Path::new(&env);


	let options = poise::FrameworkOptions::<Data, Box<dyn std::error::Error + Send + Sync>> {
		prefix_options: poise::PrefixFrameworkOptions {
			prefix: Some("!".into()),
			..poise::PrefixFrameworkOptions::default()
		},
		commands: vec![
			register(), register_global(), enable_crosspost(), ping(), disable_crosspost(), channel_status(), diagnose(),
		],
		owners: {
			// Converts newline seperated file with UIDs to hashset and ignores CLRF
			include_str!("../assets/owners.txt").replace("\r", "").split('\n').map(|x| UserId::from(u64::from_str(x).unwrap())).collect::<HashSet<UserId>>()
		},
		listener: |ctx, event, framework, user_data| {
			Box::pin(event_listener(ctx, event, framework, user_data))
		},
		..poise::FrameworkOptions::default()
	};

	let pool = SqlitePoolOptions::new()
		.max_lifetime(Duration::from_secs(6000))
		.min_connections(1)
		.max_connections(10)
		.connect(path.to_str().unwrap())
		.await.unwrap();

	poise::Framework::build()
		.token(include_str!("../assets/token.txt"))
		.user_data_setup(move |_ctx, _ready, _framework| {
			Box::pin(async move {
				Ok(
					Data {
						db: CrossDb::new(pool),
						start_date: OffsetDateTime::now_utc().unix_timestamp(),
						sys: Arc::new(RwLock::new(Sys::new()))
					}
				)
			})
		})
		.options(options)
		.run().await.unwrap();
}
