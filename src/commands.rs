use poise::serenity_prelude as serenity;
use poise::serenity_prelude::User;

use crate::DB;

// #[group]
// #[commands(register, unregister)]
// struct General;
//
// #[command]
// async fn register(ctx: &Context, msg: &Message) -> CommandResult {
// 	let lock = ctx.data.write().await;
//
// 	let lock_1 = lock.get::<DB>().unwrap();
//
// 	let mut db = lock_1.lock().await;
//
// 	let _ = sqlx::query(
// 		r#"
// 			INSERT OR REPLACE INTO channels (id, registered_author, reg_date)
// 			VALUES (?, ?, ?);
// 		"#
// 	).bind(msg.channel_id.0 as i64)
// 		.bind(msg.author.id.0 as i64)
// 		.bind(msg.timestamp.timestamp())
// 		.execute(&mut *db).await.unwrap();
//
// 	msg.reply_ping(ctx, format!("Added channel {} to tracked channels.", msg.channel_id.0)).await.unwrap();
//
// 	Ok(())
// }

type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Display your or another user's account creation date
#[poise::command(slash_command)]
pub async fn age(
	ctx: Context<'_>,
	#[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
	let user = user.as_ref().unwrap_or(ctx.author());
	ctx.say(format!("{}'s account was created at {}", user.name, user.created_at())).await?;

	Ok(())
}

#[poise::command(slash_command)]
pub async fn explode(
	ctx: Context<'_>,
	#[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
	panic!("SUS");
	Ok(())
}

/// Register slash commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
	poise::builtins::register_application_commands(ctx, global).await?;

	Ok(())
}