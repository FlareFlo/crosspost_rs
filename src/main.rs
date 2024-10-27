use std::env;
use serenity::{
	async_trait,
	model::{channel::Message, gateway::Ready},
	prelude::*,
};
use uptime_kuma_pusher::UptimePusher;

const DOMAIN: &str = "warthunder.com";
const WHITELIST: &str = include_str!("../assets/whitelist.txt");
const TOKEN: &str = include_str!("../assets/token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if WHITELIST.contains(&msg.channel_id.0.to_string()) {
			if msg.content.contains(DOMAIN) {
				match msg.crosspost(ctx).await {
					Ok(_) => {
						println!("Cross-posted message in {}", msg.channel_id.0);
					}
					Err(e) => {
						eprintln!("Failed to cross post because: {}", e);
					}
				}
			} else {
                println!("Message does not contain the domain required");
            }
		} else {
            println!("Message is not part of watched channels");
        }
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	UptimePusher::new(&env::var("UPTIME_URL").unwrap()).spawn_background();
	let token = &TOKEN.to_string().replace("\n", "");
	let mut client =
		Client::builder(token).event_handler(Handler).await.expect("Err creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}