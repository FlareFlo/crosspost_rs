mod handler;
mod commands;

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc};
use poise::builtins::{create_application_commands, register_application_commands};
use poise::serenity_prelude::{CreateApplicationCommand, CreateApplicationCommands, UserId};
use sqlx::{ConnectOptions, SqliteConnection};
use sqlx::sqlite::SqliteJournalMode;
use tokio::sync::Mutex;
use crate::commands::{enable_crosspost, ping, register, register_global};
use crate::handler::{DB, Handler};

const TOKEN: &str = include_str!("../assets/token.txt");

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
	db: Mutex<SqliteConnection>,
}

#[tokio::main]
async fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	// Fun with the path, as the path fn properly escapes paths for windows or linux
	let env = std::env::var("DATABASE_URL").unwrap().to_owned();
	let path = Path::new(&env);

	let db = sqlx::sqlite::SqliteConnectOptions::from_str(path.to_str().unwrap()).unwrap().journal_mode(SqliteJournalMode::Wal).connect().await.unwrap();

	let options = poise::FrameworkOptions::<Data, Box<dyn std::error::Error + Send + Sync>> {
		prefix_options: poise::PrefixFrameworkOptions {
			prefix: Some("!".into()),
			..Default::default()
		},
		commands: vec![
			register(), register_global(), enable_crosspost(), ping(),
		],
		owners: {
			// Converts newline seperated file with UIDs to hashset and ignores CLRF
			HashSet::from_iter(include_str!("../assets/owners.txt").replace("\r", "").split("\n").map(|x| UserId::from(u64::from_str(x).unwrap())))
		},
		..Default::default()
	};

	let db_conn = Mutex::new(db);

	poise::Framework::build()
		.token(include_str!("../assets/token.txt"))
		.user_data_setup(move |_ctx, _ready, _framework| {
			Box::pin(async move {
				Ok(
					Data {
						db: db_conn
					}
				)
			})
		})
		.options(options)
		.run().await.unwrap();
}