use poise::serenity_prelude::{Timestamp};
use crate::Context;
use crate::Error;


#[poise::command(slash_command)]
pub async fn enable_crosspost(ctx: Context<'_>) -> Result<(), Error> {
	let mut db = ctx.data().db.lock().await;

	let _ = sqlx::query(
		r#"
			INSERT OR REPLACE INTO channels (id, registered_author, reg_date)
			VALUES (?, ?, ?);
		"#
	).bind(ctx.channel_id().0 as i64).bind(ctx.author().id.0 as i64).bind(ctx.created_at().timestamp()).execute(&mut *db).await.unwrap();

	ctx.say(format!("Added channel {} to tracked channels.", ctx.channel_id().0)).await.unwrap();

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
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	let time = Timestamp::now().timestamp();
	ctx.say(format!("Pong! after {}ms", time - ctx.created_at().timestamp())).await.unwrap();

	Ok(())
}