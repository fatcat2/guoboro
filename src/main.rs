use std::convert::TryFrom;
use serenity::model::prelude::ChannelId;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::channel::ReactionType;

use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use serenity::prelude::Mentionable;
use serenity::utils::Colour;

use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction){
        let emoji: String = env::var("PIN_EMOJI").expect("token");
        let gold_reaction: ReactionType = ReactionType::try_from(emoji.as_str()).unwrap();

        if reaction.emoji == gold_reaction {
            let pin_message: Message = reaction.message(&ctx.http).await.unwrap();

            let pin_channel: u64 = env::var("PIN_CHANNEL").expect("token").parse::<u64>().unwrap();
            let embed_title = &["By ", &pin_message.author.mention().to_string(), " at [", &pin_message.timestamp.to_string(), "](", pin_message.link().as_str(), ")"].concat();

            let _msg = ChannelId(pin_channel).send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("it's a pin!");
                    e.description(&[embed_title, "\n```", pin_message.content.as_str(), "```"].concat());
                    e.colour(Colour::from_rgb(171, 35, 48));
                    e.footer(|f| {
                        f.text("pinned by @guoboro");
                        f
                    });
                    if pin_message.attachments.len() > 0{
                        e.image(pin_message.attachments[0].url.as_str());
                    }
                    e
                });
                m
            }).await;
        }
    
    }
}

#[tokio::main]
async fn main() {

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

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
