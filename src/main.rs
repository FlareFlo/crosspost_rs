mod handler;
mod commands;

use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc};
use poise::builtins::{create_application_commands, register_application_commands};
use poise::serenity_prelude::{CreateApplicationCommand, CreateApplicationCommands, UserId};
use sqlx::{ConnectOptions};
use sqlx::sqlite::SqliteJournalMode;
use tokio::sync::Mutex;
use crate::commands::{age, do_trolling, register, register_global};
use crate::handler::{DB, Handler};

const TOKEN: &str = include_str!("../assets/token.txt");

#[tokio::main]
async fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	// Fun with the path, as the path fn properly escapes paths for windows or linux
	let env = std::env::var("DATABASE_URL").unwrap().to_owned();
	let path = Path::new(&env);

	let db = sqlx::sqlite::SqliteConnectOptions::from_str(path.to_str().unwrap()).unwrap().journal_mode(SqliteJournalMode::Wal).connect().await.unwrap();

	let options = poise::FrameworkOptions {
		prefix_options: poise::PrefixFrameworkOptions {
			prefix: Some("!".into()),
			..Default::default()
		},
		commands: vec![
			register(), do_trolling(), register_global(), register()
		],
		owners: {
			// Converts newline seperated file with UIDs to hashset and ignores CLRF
			HashSet::from_iter(include_str!("../assets/owners.txt").replace("\r", "").split("\n").map(|x|UserId::from(u64::from_str(x).unwrap())))
		},
		..Default::default()
	};

	let mut framework = poise::Framework::build()
		.token(include_str!("../assets/token.txt"))
		.options(options)
		.user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(()) }));

	let serenity = framework.build().await.unwrap();

	serenity.start().await.unwrap();
}