use std::sync::{Arc};
use serenity::{async_trait, model::{channel::Message, gateway::Ready}};
use serenity::prelude::{Context, EventHandler};
use sqlx::{SqliteConnection};
use tokio::sync::Mutex;

pub struct Handler {
	pub db: Arc<Mutex<SqliteConnection>>,
}

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, _ctx: Context, msg: Message) {
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