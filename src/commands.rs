use serenity::prelude::*;
use serenity::{
	framework::standard::{
		macros::{command, group},
		CommandResult,
	},
	model::{
		channel::{Message},
	},
};
use crate::DB;


#[group]
#[commands(ping, register, unregister)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	let delta_t = time::OffsetDateTime::now_utc().unix_timestamp() - msg.timestamp.timestamp();
	msg.reply_ping(ctx, format!("Pong after {delta_t}ms")).await.unwrap();

	Ok(())
}

#[command]
async fn register(ctx: &Context, msg: &Message) -> CommandResult {
	let lock = ctx.data.write().await;

	let lock_1 = lock.get::<DB>().unwrap();

	let mut db = lock_1.lock().await;

	let _ = sqlx::query(
		r#"
			INSERT OR REPLACE INTO channels (id, registered_author, reg_date)
			VALUES (?, ?, ?);
		"#
	).bind(msg.channel_id.0 as i64)
		.bind(msg.author.id.0 as i64)
		.bind(msg.timestamp.timestamp())
		.execute(&mut *db).await.unwrap();

	msg.reply_ping(ctx, format!("Added channel {} to tracked channels.", msg.channel_id.0)).await.unwrap();

	Ok(())
}

#[command]
async fn unregister(ctx: &Context, msg: &Message) -> CommandResult {
	let lock = ctx.data.write().await;

	let lock_1 = lock.get::<DB>().unwrap();

	let mut db = lock_1.lock().await;

	let _ = sqlx::query(
		r#"
			DELETE
			FROM channels
			WHERE id = ?;
		"#
	).bind(msg.channel_id.0 as i64)
		.execute(&mut *db).await.unwrap();

	msg.reply_ping(ctx, format!("Removed channel {} from tracked channels.", msg.channel_id.0)).await.unwrap();

	Ok(())
}