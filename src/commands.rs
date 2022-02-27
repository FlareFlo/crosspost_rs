use poise::serenity_prelude::{Channel, Timestamp};
use crate::Context;
use crate::Error;


#[poise::command(slash_command)]
pub async fn enable_crosspost(ctx: Context<'_>, #[description = "The channel that will be enabled"] channel: Channel) -> Result<(), Error> {
	let mut db = ctx.data().db.lock().await;

	let _ = sqlx::query(
		r#"
			INSERT OR REPLACE INTO channels (id, registered_author, reg_date)
			VALUES (?, ?, ?);
		"#
	).bind(channel.id().0 as i64).bind(ctx.author().id.0 as i64).bind(ctx.created_at().timestamp()).execute(&mut *db).await.unwrap();

	ctx.say(format!("Added channel {} to tracked channels.", channel.id().0)).await.unwrap();

	Ok(())
}

#[poise::command(slash_command)]
pub async fn disable_crosspost(ctx: Context<'_>, #[description = "The channel that will be disabled"] channel: Channel) -> Result<(), Error> {
	let mut db = ctx.data().db.lock().await;

	let _ = sqlx::query(
		r#"
			DELETE
			FROM channels
			WHERE id = ?;
		"#
	).bind(channel.id().0 as i64).execute(&mut *db).await.unwrap();

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