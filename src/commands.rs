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


#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	let delta_t = time::OffsetDateTime::now_utc().unix_timestamp() - msg.timestamp.timestamp();
	msg.reply_ping(ctx, format!("Pong after {delta_t}ms")).await.unwrap();

	Ok(())
}