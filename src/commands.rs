use poise::ReplyHandle;
use poise::serenity_prelude::{CacheHttp, Channel, Color, Timestamp, UserId};
use sqlx::Row;
use sysinfo::{ProcessorExt, ProcessRefreshKind, RefreshKind, SystemExt};

use crate::Context;
use crate::Error;
use crate::info::HOSTNAME;
use crate::info::OS;

#[poise::command(slash_command)]
pub async fn enable_crosspost(ctx: Context<'_>, #[description = "The channel that will be enabled"] channel: Channel) -> Result<(), Error> {
	ctx.data().db.channel_enable_crosspost(
		channel.id().0 as i64,
		ctx.author().id.0 as i64,
		ctx.created_at().timestamp(),
		channel.clone().guild().unwrap().guild_id.0 as i64
	).await;
	ctx.say(format!("Added channel {} to tracked channels.", channel)).await.unwrap();

	Ok(())
}

#[poise::command(slash_command)]
pub async fn disable_crosspost(ctx: Context<'_>, #[description = "The channel that will be disabled"] channel: Channel) -> Result<(), Error> {
	ctx.data().db.channel_disable_crosspost(ctx.channel_id().0 as i64).await;

	ctx.say(format!("Removed channel {} from tracked channels.", channel)).await.unwrap();

	Ok(())
}

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
	poise::builtins::register_application_commands(ctx, global).await?;

	Ok(())
}

#[poise::command(prefix_command, hide_in_help)]
pub async fn register_global(ctx: Context<'_>) -> Result<(), Error> {
	poise::builtins::register_application_commands(ctx, true).await?;

	Ok(())
}

#[poise::command(slash_command)]
/// Total time from invoking the command to this message arriving
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	let time = Timestamp::now().timestamp();
	ctx.say(format!("Pong! after {}ms", time - ctx.created_at().timestamp())).await.unwrap();

	Ok(())
}

#[poise::command(slash_command)]
pub async fn channel_status(ctx: Context<'_>, #[description = "The target channel to show the status of"] channel: Channel) -> Result<(), Error> {
	if let Ok(result) = ctx.data().db.channel_get(channel).await {
		if !result.is_empty() {
			let author: i64 = result.try_get("registered_author").unwrap_or(0);
			let stamp_of_registry: i64 = result.try_get("reg_date").unwrap_or(0);

			let author_name = UserId::from(author as u64).to_user(ctx.discord()).await.unwrap();

			ctx.say(format!("This channel was registered by {author_name} on <t:{stamp_of_registry}>")).await.unwrap();

			return Ok(());
		}
	}
	ctx.say("This channel is not registered").await.unwrap();

	Ok(())
}

#[poise::command(slash_command)]
pub async fn diagnose(ctx: Context<'_>) -> Result<(), Error> {
	let sys = ctx.data().sys.write().await.get_sysinfo();
	ctx.send(|b|{
		b.embed(|e|{
			e.description("The current status of the bot").color(Color::from_rgb(199, 10, 75))
				.field("Online since", format!("<t:{}:R>", ctx.data().start_date), false)
		}).embed(|e|{
			e.description(format!("About: \"{}\"", HOSTNAME.to_owned())).color(Color::from_rgb(199, 10, 75))
				.field("CPU usage", format!("{:.1}%", sys.cpu_usage), false)
				.field("Free memory", format!("{:?} mb", sys.avail_mem), false)
				.field("OS", OS.to_owned(), false)
		})
	}).await.unwrap();

	Ok(())
}