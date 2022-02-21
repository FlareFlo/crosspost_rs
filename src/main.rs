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
use crate::commands::GENERAL_GROUP;
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

	let framework =  StandardFramework::new()
		.configure(|c| c
			.with_whitespace(true)
			.prefix("!")
		).group(&GENERAL_GROUP);

	let mut client = Client::builder(token)
		.event_handler(handler)
		.framework(framework)
		.await.expect("Err creating client");

	{
		let mut data = client.data.write().await;
		data.insert::<DB>(Arc::new(Mutex::new(db)));
	}

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}