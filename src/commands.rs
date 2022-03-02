use poise::serenity_prelude::{Channel, Timestamp, UserId};
use sqlx::Row;

use crate::Context;
use crate::Error;

#[poise::command(slash_command)]
pub async fn enable_crosspost(ctx: Context<'_>, #[description = "The channel that will be enabled"] channel: Channel) -> Result<(), Error> {
	ctx.data().db.channel_enable_crosspost(ctx.id() as i64, ctx.author().id.0 as i64, ctx.created_at().timestamp()).await;
	ctx.say(format!("Added channel {} to tracked channels.", channel.id().0)).await.unwrap();

	Ok(())
}

#[poise::command(slash_command)]
pub async fn disable_crosspost(ctx: Context<'_>, #[description = "The channel that will be disabled"] channel: Channel) -> Result<(), Error> {
	ctx.data().db.channel_disable_crosspost(ctx.channel_id().0 as i64).await;

	ctx.say(format!("Removed channel {} from tracked channels.", channel.id().0)).await.unwrap();

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