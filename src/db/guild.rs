use std::process::id;
use poise::serenity_prelude::{Guild, Message};
use sqlx::Row;
use sqlx::SqliteConnection;
use time::OffsetDateTime;
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
	pub async fn guild_get_warn_level(&self, guild: &Guild) -> Option<i64> {
		let result = sqlx::query(
			r#"
						SELECT whitelisted
						FROM guilds
						WHERE id = ?
					"#
		).bind(guild.id.0 as i64).fetch_one(&self.db).await.unwrap();

		return match result.try_get("whitelisted") {
			Ok(val) => {
				Some(val)
			}
			_ => {
				None
			}
		};
	}
	pub async fn guild_bump_spam_count(&self) {
		const HOUR: i64 = 1000 * 60 * 60;
		const LIMIT_PER_HOUR: i64 = 50;
		let curr_time = OffsetDateTime::now_utc().unix_timestamp();

		let res = sqlx::query(
			r#"
					SELECT id
					FROM (
        		    	 SELECT guilds.id, COUNT(messages.id) as messages
        				 FROM guilds
           			     JOIN channels on guilds.id = channels.guild_id
              			 JOIN messages on channels.id = messages.channel_id
         				 WHERE messages.date_received BETWEEN ? AND ?
         				 GROUP BY guilds.id
         				 )
					WHERE messages > ?
				"#
		).bind(curr_time).bind(curr_time - HOUR).bind(LIMIT_PER_HOUR).fetch_all(&self.db).await.unwrap();

		let bad_ids = res.iter().map(|x| x.try_get("id").unwrap()).collect::<Vec<i64>>();

		for bad_id in bad_ids {
			let _ = sqlx::query(
				r#"
					UPDATE guilds
					SET whitelisted = whitelisted + 1
					WHERE id = ?
			"#
			).bind(bad_id).execute(&self.db).await.unwrap();
		}
	}

	pub async fn guild_get_bad_guilds(&self) -> Vec<u64> {
		let res = sqlx::query(
			r#"
					SELECT id
					FROM guilds
					WHERE whitelisted >= 3
				"#
		).fetch_all(&self.db).await.unwrap();

		let mut items = Vec::new();
		for item in res {
			let num: i64 = item.try_get("id").unwrap();
			items.push(num as u64);
		}
		items
	}
}