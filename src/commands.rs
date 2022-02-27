use poise::serenity_prelude::{CacheHttp, Channel, Timestamp, User, UserId};
use sqlx::Row;
use time::{Date, OffsetDateTime, PrimitiveDateTime};
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

#[poise::command(slash_command)]
pub async fn status(ctx: Context<'_>, #[description = "The target channel to show the status of"] channel: Channel) -> Result<(), Error> {
	let mut db = ctx.data().db.lock().await;


	let res = sqlx::query(
		r#"
			SELECT *
			FROM channels
			WHERE id = ?;
		"#
	).bind(channel.id().0 as i64).fetch_one(&mut *db).await;

	if let Ok(result) = res {
		if !result.is_empty() {
			let author: i64 = result.try_get("registered_author").unwrap_or(0);
			let stamp_of_registry: i64 = result.try_get("reg_date").unwrap_or(0);

			let date_of_registry = OffsetDateTime::from_unix_timestamp(stamp_of_registry.into()).unwrap().date();
			let author_name = UserId::from(author as u64).to_user(ctx.discord()).await.unwrap();

			ctx.say(format!("This channel was registered by {author_name} on {date_of_registry}")).await.unwrap();

			return Ok(());
		}
	}
	ctx.say("This channel is not registered").await.unwrap();

	Ok(())
}