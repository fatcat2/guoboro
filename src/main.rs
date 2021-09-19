use std::convert::TryFrom;
use std::collections::HashMap;

use cached::proc_macro::cached;
use cached::Return;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::http::CacheHttp;
use serenity::http::client::Http;
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::channel::ReactionType;
use serenity::model::id::{ChannelId, GuildId, MessageId};

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
#[commands(pin)]
struct Pin;

#[group]
#[commands(shibe)]
struct Shibe;

#[cached(size=100, with_cached_flag = true)]
async fn get_nickname(user_id: u64, guild_id: u64) -> Return<String>{
    let token = env::var("DISCORD_TOKEN").expect("token");
    let http: Http = Http::new_with_token(&token);
    let nick: String = match GuildId(guild_id).member(http, user_id).await {
        Ok(x) => match x.nick {
            Some(nickname) => nickname,
            None => x.user.name
        },
        Err(e) => e.to_string()
    };
    Return::new(nick)
}

#[command]
async fn shibe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<String>()?;
    let second = args.single::<String>()?;

    if first != "gibe" && second != "pls" {
        msg.reply(&ctx.http, "no >:(").await.unwrap();
    }

    let mut resp = reqwest::get("https://shibe.online/api/shibes?count=1&urls=true&httpsUrls=true")
        .await?
        .json::<Vec<String>>()
        .await?;
    msg.channel_id.say(&ctx.http, resp.pop().unwrap()).await.unwrap();
    Ok(())
}

#[command]
async fn pin(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<String>()?;
    let second = args.single::<u64>()?;

    let ids = first.split("/").collect::<Vec<&str>>();
    let message_id = ids.get(&ids.len() - 1).unwrap();
    let channel_id = ids.get(&ids.len() - 2).unwrap();

    let channel: ChannelId = ChannelId(channel_id.parse::<u64>().unwrap());

    let mut messages: Vec<Message> = vec![channel.message(&ctx.http, MessageId(message_id.parse::<u64>().unwrap())).await.unwrap()];
    let mut after_messages: Vec<Message>  = channel.messages(&ctx.http, |retriever| {
        retriever.after(MessageId(message_id.parse::<u64>().unwrap()))
            .limit(second)
    }).await.unwrap();

    messages.append(&mut after_messages);
    
    let mut nicks: HashMap<String, String> = HashMap::new();
    for message in messages.clone() {
        let nick: Return<String> = get_nickname(*message.author.id.as_u64(), *msg.guild_id.unwrap().as_u64()).await;
        nicks.insert(message.author.id.to_string(), nick.to_string());
    }

    messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let message_block: String = messages.iter().map(|msg| {
        let mut line = nicks.get(&msg.author.id.to_string()).unwrap().clone();
        line.push_str(": ");
        line.push_str(msg.content.as_str());
        line.push_str("\n");
        line
    }).collect::<Vec<String>>().concat();


    let pin_channel: u64 = env::var("PIN_CHANNEL").expect("token").parse::<u64>().unwrap();
    let embed_title = &[
        "By ",&msg.author.mention().to_string(),
        " at [", &msg.timestamp.to_string(),"](",messages[0].link().as_str(),")"
        ].concat();
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

            let mut count = 0;

            for reaction in &pin_message.reactions {
                if reaction.reaction_type == gold_reaction {
                    count = reaction.count;
                    break;
                }
            };

            if count != 1 {
                return;
            }

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
