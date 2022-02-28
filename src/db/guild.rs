use poise::serenity_prelude::{Guild, Message};
use sqlx::Row;
use sqlx::SqliteConnection;
use crate::CrossDb;

impl CrossDb {
	pub async fn guild_add(&self, guild: &Guild) {
		let _ = sqlx::query(
			r#"
					INSERT INTO guilds (id, join_date)
					VALUES (? , ?);
				"#
		).bind(guild.id.0 as i64).bind(guild.joined_at.unix_timestamp()).execute(&self.db).await.unwrap();
	}
	pub async fn guild_is_whitlisted(&self, guild: &Guild) -> bool {
		let result = sqlx::query(
			r#"
						SELECT whitelisted
						FROM guilds
						WHERE id = ?
					"#
		).bind(guild.id.0 as i64).fetch_one(&self.db).await.unwrap();

		return match result.try_get("whitelisted") {
			Ok(0) => {
				true
			}
			_ => {
				false
			}
		};
	}
}