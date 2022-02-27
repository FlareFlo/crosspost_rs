use poise::serenity_prelude::Context;
use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn event_listener(
	ctx: &Context,
	event: &poise::Event<'_>,
	_framework: &poise::Framework<Data, Error>,
	_user_data: &Data,
) -> Result<(), Error> {
	match event {
		poise::Event::Ready { data_about_bot } => {
			println!("{} is connected!", data_about_bot.user.name)
		}
		poise::Event::Message { new_message } => {
				new_message.crosspost(ctx).await.unwrap();
		}
		_ => {}
	}

	Ok(())
}