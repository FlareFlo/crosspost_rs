use std::sync::{Arc};
use serenity::{async_trait, model::{channel::Message, gateway::Ready}};
use serenity::prelude::{Context, EventHandler, TypeMapKey};
use sqlx::{SqliteConnection};
use tokio::sync::Mutex;

pub struct Handler {
}

pub struct DB;

impl TypeMapKey for DB {
	type Value = Arc<Mutex<SqliteConnection>>;
}

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content.contains("store") {
			let lock = ctx.data.write().await;

			let lock_1 = lock.get::<DB>().unwrap();

			let mut db = lock_1.lock().await;

			let _ = sqlx::query(
				r#"
		INSERT INTO messages (author, date_received)
		VALUES (?, ?);
		"#
			).bind(msg.author.id.0 as i64)
				.bind(msg.timestamp.timestamp())
				.execute(&mut *db)
				.await.unwrap();
		}
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}