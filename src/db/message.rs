use poise::serenity_prelude::Message;

use crate::CrossDb;

impl CrossDb {
	pub async fn messages_log_message(&self, new_message: &Message) {
		let _ = sqlx::query(
			r#"
				INSERT INTO messages (id, author, date_received, channel_id)
				VALUES (?, ?, ?, ?);
			"#
		).bind(new_message.id.0 as i64)
			.bind(new_message.author.id.0 as i64)
			.bind(new_message.timestamp.unix_timestamp())
			.bind(new_message.channel_id.0 as i64)
			.execute(&self.db)
			.await.unwrap();
	}
}