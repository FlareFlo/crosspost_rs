use serenity::{
	async_trait,
	model::{channel::Message, gateway::Ready},
	prelude::*,
};

const DOMAIN: &str = "warthunder.com";
const WHITELIST: &str = include_str!("../assets/whitelist.txt");
const TOKEN: &str = include_str!("../assets/token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
			println!("{}", msg.content);
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	let token = &TOKEN.to_string().replace("\n", "");
	let mut client = Client::builder(token).event_handler(Handler).await.expect("Err creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}