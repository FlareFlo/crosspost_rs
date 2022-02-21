use std::cell::UnsafeCell;
use std::str::FromStr;
use std::sync::{Arc, MutexGuard};
use serenity::{async_trait, Client, model::{channel::Message, gateway::Ready}};
use serenity::prelude::{Context, EventHandler};
use sqlx::{ConnectOptions, SqliteConnection};
use sqlx::sqlite::SqliteJournalMode;
use tokio::sync::Mutex;

const TOKEN: &str = include_str!("../assets/token.txt");

struct Handler {
	db: Arc<Mutex<SqliteConnection>>,
}

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
			println!("{}", msg.content);

		let mut lock = self.db.lock().await;

		let _ = sqlx::query(
			r#"
		INSERT INTO messages (author, date_received)
		VALUES (?, ?);
	"#
		).bind(msg.author.id.0 as i64)
			.bind(msg.timestamp.timestamp())
			.execute(&mut *lock)
			.await.unwrap();
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	let mut db = sqlx::sqlite::SqliteConnectOptions::from_str(&std::env::var("DATABASE_URL").unwrap()).unwrap()
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