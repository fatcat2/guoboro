use std::convert::TryFrom;

use std::collections::HashMap;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::channel::ReactionType;
use serenity::model::id::{ChannelId, MessageId};

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

use serenity::framework::standard::Args;

use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[group]
// Sets a single prefix for this group.
// So one has to call commands in this group
// via `~math` instead of just `~`.
#[commands(pin)]
struct Pin;

#[command]
// Lets us also call `~math *` instead of just `~math multiply`.
async fn pin(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<String>()?;
    let second = args.single::<u64>()?;

    let ids = first.split("/").collect::<Vec<&str>>();
    let message_id = ids.get(&ids.len() - 1).unwrap();
    let channel_id = ids.get(&ids.len() - 2).unwrap();

    let channel: ChannelId = ChannelId(channel_id.parse::<u64>().unwrap());
    
    let messages: Vec<Message>  = channel.messages(&ctx.http, |retriever| {
        retriever.after(MessageId(message_id.parse::<u64>().unwrap()))
            .limit(second)
    }).await.unwrap();

    let mut nicks: HashMap<String, String> = HashMap::new();
    for message in messages.clone().into_iter() {
        let nick: String = match message.author_nick(&ctx.http).await {
            Some(x) => x,
            None => message.author.name
        };
        nicks.insert(message.author.id.to_string(), nick);
    }


    let message_block: String = messages.iter().map(|msg| {
        let mut line = nicks.get(&msg.author.id.to_string()).unwrap().clone();
        line.push_str(": ");
        line.push_str(msg.content.as_str());
        line.push_str("\n");
        line
    }).collect::<Vec<String>>().concat();
   

    let pin_channel: u64 = env::var("PIN_CHANNEL").expect("token").parse::<u64>().unwrap();
    let embed_title = &["By ", &msg.author.mention().to_string(), " at [", &msg.timestamp.to_string(), "](", messages[0].link().as_str(), ")"].concat();
    //let res = first * second;
    let _msg = ChannelId(pin_channel).send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("it's a pin!");
                    e.description(&[embed_title, "\n```", &message_block.as_str(), "```"].concat());
                    e.colour(Colour::from_rgb(171, 35, 48));
                    e.footer(|f| {
                        f.text("pinned by @guoboro");
                        f
                    });
                    e
                });
                m
            }).await;

    Ok(())
}

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
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP)
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
