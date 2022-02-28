use poise::serenity_prelude::Context;
use sqlx::{Row, Sqlite};
use sqlx::migrate::Migrate;
use crate::Data;
use crate::db::cross_db::CrossDb;

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
			if user_data.db.channel_is_in_watched_channel(new_message).await {
				new_message.crosspost(ctx).await;

				user_data.db.log_msg(new_message);
			}

		}
		poise::Event::GuildCreate { guild, is_new } => {
			println!("{} {:?}", guild.name, is_new);

			if *is_new {
				user_data.db.guild_add(guild).await;
			} else {
				if !user_data.db.guild_is_whitlisted(guild).await {
					guild.leave(ctx).await.unwrap();
				}
			}
		}
		_ => {}
	}

	Ok(())
}