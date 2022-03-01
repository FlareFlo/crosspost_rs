use std::time::Duration;
use poise::serenity_prelude::{CacheHttp, Context};
use sqlx::{Row, Sqlite};
use sqlx::migrate::Migrate;
use crate::Data;
use crate::db::cross_db::CrossDb;

type Error = Box<dyn std::error::Error + Send + Sync>;

const MAX_WARN: i32 = 3;

pub async fn event_listener(
	ctx: &Context,
	event: &poise::Event<'_>,
	_framework: &poise::Framework<Data, Error>,
	user_data: &Data,
) -> Result<(), Error> {
	match event {
		poise::Event::Ready { data_about_bot } => {
			println!("{} is connected!", data_about_bot.user.name);
			let user_data_ref = user_data.clone();
			let ctx_ref = ctx.clone();
			tokio::task::spawn(async move {
				loop {
					user_data_ref.db.guild_bump_spam_count();
					let bad_guilds = user_data_ref.db.guild_get_bad_guilds().await;
					for bad_guild in bad_guilds {
						&ctx_ref.http().get_guild(bad_guild).await.unwrap().leave(&ctx_ref).await.unwrap();
					}
					tokio::time::sleep(Duration::from_secs(60 * 60)).await;
				}
			});
		}
		poise::Event::Message { new_message } => {
			if user_data.db.channel_is_in_watched_channel(new_message).await {
				new_message.crosspost(ctx).await;

				user_data.db.messages_log_message(new_message);
			}
		}
		poise::Event::GuildCreate { guild, is_new } => {
			println!("{} {:?}", guild.name, is_new);

			if *is_new {
				user_data.db.guild_add(guild).await;
			} else {
				if user_data.db.guild_get_warn_level(guild).await >= Some(MAX_WARN.into()) {
					guild.leave(ctx).await.unwrap();
				}
			}
		}
		_ => {}
	}

	Ok(())
}