use poise::serenity_prelude::Message;
use sqlx::Row;
use sqlx::SqliteConnection;
use crate::CrossDb;


impl CrossDb {
	pub async fn messages_log_message(&self, new_message: &Message) {
		let _ = sqlx::query(
			r#"
				INSERT INTO messages (author, date_received)
				VALUES (?, ?);
			"#
		).bind(new_message.author.id.0 as i64).bind(new_message.timestamp.unix_timestamp()).execute(&self.db).await.unwrap();
	}
}