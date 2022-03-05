use poise::serenity_prelude::{CacheHttp, ChannelType, Context};

use crate::Data;

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
					user_data_ref.db.guild_bump_spam_count().await;
					let bad_guilds = user_data_ref.db.guild_get_bad_guilds().await;
					for bad_guild in bad_guilds {
						let _ = &ctx_ref.http().get_guild(bad_guild).await.unwrap().leave(&ctx_ref).await.unwrap();
					}
					tokio::time::sleep(std::time::Duration::from_secs(60 * 60)).await;
				}
			});
		}
		poise::Event::Message { new_message } => {
			if let Ok(ok) = new_message.channel(ctx).await {
				if ok.guild().unwrap().kind == ChannelType::News {
					if user_data.db.channel_is_in_watched_channel(new_message).await {
						match new_message.crosspost(ctx).await {
							Ok(_) => {
								user_data.db.messages_log_message(new_message).await;
							}
							_ => {}
						}
					}
				}
			}
		}
		poise::Event::GuildCreate { guild, is_new } => {
			println!("{} {:?}", guild.name, is_new);

			if user_data.db.guild_get_warn_level(guild).await >= Some(MAX_WARN.into()) {
				const RATE_URL: &str = r#"https://github.com/FlareFlo/crosspost_rs/blob/master/rate.md"#;
				guild.owner_id.create_dm_channel(ctx).await.unwrap().say(ctx, format!("Your server \"{}\" has been blacklisted due to exceeding the rate limit. Find out more at {}", guild.name, RATE_URL)).await.unwrap();
				guild.leave(ctx).await.unwrap();
			} else {
				user_data.db.guild_add(guild).await;
			}
		}
		_ => {}
	}

	Ok(())
}