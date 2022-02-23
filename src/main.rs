mod handler;
mod commands;

use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc};
use poise::builtins::register_application_commands;
use sqlx::{ConnectOptions};
use sqlx::sqlite::SqliteJournalMode;
use tokio::sync::Mutex;
use crate::commands::{age, explode, register};
use crate::handler::{DB, Handler};

const TOKEN: &str = include_str!("../assets/token.txt");

#[tokio::main]
async fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	// Fun with the path, as the path fn properly escapes paths for windows or linux
	let env = std::env::var("DATABASE_URL").unwrap().to_owned();
	let path = Path::new(&env);

	let db = sqlx::sqlite::SqliteConnectOptions::from_str(path.to_str().unwrap()).unwrap()
		.journal_mode(SqliteJournalMode::Wal)
		.connect().await.unwrap();

		poise::Framework::build()
			.token(include_str!("../assets/token.txt"))
			.user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move {
				register_application_commands(_ctx, true).await.unwrap();
				Ok(())
			}))
			.options(poise::FrameworkOptions {
				prefix_options: poise::PrefixFrameworkOptions {
					prefix: Some("!".into()),
					..Default::default()
				},
				commands: vec![register(), explode()],
				..Default::default()
			})
			.run().await.unwrap();
}