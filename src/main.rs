mod handler;
mod commands;

use std::str::FromStr;
use std::sync::{Arc};
use serenity::{Client};
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::prelude::TypeMapKey;
use sqlx::{ConnectOptions};
use sqlx::sqlite::SqliteJournalMode;
use tokio::sync::Mutex;
use crate::commands::{age, explode, register};
use crate::handler::{DB, Handler};

const TOKEN: &str = include_str!("../assets/token.txt");

#[tokio::main]
async fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	let db = sqlx::sqlite::SqliteConnectOptions::from_str(&std::env::var("DATABASE_URL").unwrap()).unwrap()
		.journal_mode(SqliteJournalMode::Wal)
		.connect().await.unwrap();

	let token = &TOKEN.to_string().replace("\n", "");

	let handler = Handler {
	};

	poise::Framework::build()
		.token(include_str!("../assets/token.txt"))
		.user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(()) }))
		.options(poise::FrameworkOptions {
			// configure framework here
			prefix_options: poise::PrefixFrameworkOptions {
				prefix: Some("!".into()),
				..Default::default()
			},
			commands: vec![age(), register(), explode()],
			..Default::default()
		})
		.run().await.unwrap();
}