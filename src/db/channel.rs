use std::process::id;
use poise::serenity_prelude::{Channel, Message};
use sqlx::{Row, SqliteConnection};
use poise::serenity_prelude::Context;
use sqlx::sqlite::SqliteRow;
use crate::{CrossDb, Data};

impl CrossDb {
	pub async fn channel_enable_crosspost(&self, id: i64, author: i64, crated: i64) {
		let _ = sqlx::query(
			r#"
			INSERT OR REPLACE INTO channels (id, registered_author, reg_date)
			VALUES (?, ?, ?);
		"#
		).bind(id).bind(author).bind(crated).execute(&mut *self.db.lock().await).await.unwrap();
	}
	pub async fn channel_disable_crosspost(&self, id: i64) {
		let _ = sqlx::query(
			r#"
			DELETE
			FROM channels
			WHERE id = ?;
		"#
		).bind(id).execute(&mut *self.db.lock().await).await.unwrap();
	}
	pub async fn channel_get(&self, channel: Channel) -> Result<SqliteRow, sqlx::Error> {
		sqlx::query(
			r#"
			SELECT *
			FROM channels
			WHERE id = ?;
		"#
		).bind(channel.id().0 as i64).fetch_one(&mut *self.db.lock().await).await
	}
	pub async fn channel_is_in_watched_channel(&self, channel_id: &Message) -> bool {
		let result = sqlx::query(
			r#"
					SELECT COUNT(*) AS count
					FROM channels
					WHERE id = ?;
					"#
		).bind(channel_id.channel_id.0 as i64).fetch_one(&mut *self.db.lock().await).await.unwrap();

		return match result.try_get("count") {
			Ok(1) => {
				true
			}
			_ => {
				false
			}
		};
	}
}