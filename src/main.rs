use serenity::async_trait;
use serenity::model::channel::ReactionType;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        let channel_id = Channel ID UwU;

        if msg.channel_id == channel_id {
            if let Err(why) = msg.react(_ctx.http.clone(), ReactionType::Unicode("❤️".to_string())).await {
                println!("Failed to add reaction: {:?}", why);
            }
           println!("Oh Yea");
       }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token
    let token = "Token UwU";

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
