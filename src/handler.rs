use poise::serenity_prelude::Context;
use sqlx::{Row, Sqlite};
use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn event_listener(
	ctx: &Context,
	event: &poise::Event<'_>,
	_framework: &poise::Framework<Data, Error>,
	user_data: &Data,
) -> Result<(), Error> {
	match event {
		poise::Event::Ready { data_about_bot } => {
			println!("{} is connected!", data_about_bot.user.name)
		}
		poise::Event::Message { new_message } => {
			let mut db = user_data.db.lock().await;

			let result = sqlx::query(
				r#"
						SELECT COUNT(*) AS count
						FROM channels
						WHERE id = ?;
					"#
			).bind(new_message.channel_id.0 as i64).fetch_one(&mut *db).await.unwrap();

			match result.try_get("count") {
				Ok(1) => {
					new_message.crosspost(ctx).await;

					let _ = sqlx::query(
						r#"
							INSERT INTO messages (author, date_received)
							VALUES (?, ?);
						"#
					).bind(new_message.author.id.0 as i64)
						.bind(new_message.timestamp.unix_timestamp())
						.execute(&mut *db).await.unwrap();
				}
				_ => {
				}
			}
		}
		_ => {}
	}

	Ok(())
}