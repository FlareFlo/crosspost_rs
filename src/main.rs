use std::str::FromStr;
use serenity::{
	async_trait,
	model::{channel::Message, gateway::Ready},
	prelude::*,
};
use sqlx::{ConnectOptions};
use sqlx::sqlite::SqliteJournalMode;

const TOKEN: &str = include_str!("../assets/token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
			println!("{}", msg.content);
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

	let query = sqlx::query!(
		r#"
		INSERT INTO messages (author, date_received)
		VALUES (?, ?);
	"#,
	0_i32 ,0_i32
	);

	query.fetch_one(&mut db).await.unwrap();
	// let token = &TOKEN.to_string().replace("\n", "");
	// let mut client = Client::builder(token).event_handler(Handler).await.expect("Err creating client");
	//
	// if let Err(why) = client.start().await {
	// 	println!("Client error: {:?}", why);
	// }
}