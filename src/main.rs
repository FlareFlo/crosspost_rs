use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use uptime_kuma_pusher::UptimePusher;
const WHITELIST: &str = include_str!("../assets/whitelist.txt");
const TOKEN: &str = include_str!("../assets/token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!(
            "Got message from: {} in {:?}",
            msg.author.name,
            msg.channel_id.name(&ctx).await
        );
        if WHITELIST.contains(&msg.channel_id.get().to_string()) {
            match msg.crosspost(&ctx).await {
                Ok(_) => {
                    println!("Cross-posted message in {}", msg.channel_id.get());
                }
                Err(e) => {
                    eprintln!("Failed to cross post because: {}", e);
                }
            }
        } else {
            println!(
                "Message is not part of watched channels: {}",
                msg.channel_id.get()
            );
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// Use current thread due to: https://github.com/mozilla/sccache/issues/1972
#[tokio::main(flavor = "current_thread")]
async fn main() {
    UptimePusher::new(&env::var("UPTIME_URL").unwrap(), false).spawn_background();
    println!("{}", "Spawned uptime pusher");
    let token = &TOKEN.to_string().replace("\n", "");
    let mut client = Client::builder(token, GatewayIntents::non_privileged())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
