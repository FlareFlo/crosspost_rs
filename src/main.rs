mod handler;

use std::cell::UnsafeCell;
use std::str::FromStr;
use std::sync::{Arc, MutexGuard};
use serenity::{async_trait, Client, model::{channel::Message, gateway::Ready}};
use serenity::prelude::{Context, EventHandler};
use sqlx::{ConnectOptions, SqliteConnection};
use sqlx::sqlite::SqliteJournalMode;
use tokio::sync::Mutex;
use crate::handler::Handler;

const TOKEN: &str = include_str!("../assets/token.txt");

#[tokio::main]
async fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	let db = sqlx::sqlite::SqliteConnectOptions::from_str(&std::env::var("DATABASE_URL").unwrap()).unwrap()
		.journal_mode(SqliteJournalMode::Wal)
		.connect().await.unwrap();

	let token = &TOKEN.to_string().replace("\n", "");

	let handler = Handler {
		db: Arc::new(Mutex::new(db))
	};
	let mut client = Client::builder(token).event_handler(handler).await.expect("Err creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}