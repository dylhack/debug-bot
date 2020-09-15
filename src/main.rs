use std::env::{var};
use serenity::client::{Client};
use serenity::prelude::{Context, EventHandler};
use serenity::model::prelude::{Message, Ready};

struct Handler;

const PREFIX: &'static str = "$dbg";

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if !msg.content.starts_with(PREFIX) || msg.author.bot {
			return;
		}

		let serialized = serde_json::to_string_pretty(&msg)
			.expect("Failed to serialize");
		msg.channel_id.say(
			ctx, 
			format!("```json\n{}\n```", serialized),
		).await.unwrap();
	}

	async fn ready(&self, _ctx: Context, rdy: Ready) {
		println!("Ready as {}", rdy.user.tag());
	}
}


#[tokio::main]
async fn main() {
	let token = var("DISCORD_TOKEN")
		.expect("DISCORD_TOKEN not provided");
	let mut client = Client::new(&token)
		.event_handler(Handler)
		.await
		.expect("Failed to build client");

	client.start().await.unwrap();
}
