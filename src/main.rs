use serenity::client::{Client, Context};
use serenity::model::channel::Message;

use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;

mod shibe;
use shibe::SHIBE_GROUP;

mod pin;
use pin::PIN_GROUP;

mod reaction;
use reaction::Handler;

#[group]
#[commands(ping)]
struct General;



#[tokio::main]
async fn main() {

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP)
        .group(&SHIBE_GROUP)
        .group(&PIN_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
