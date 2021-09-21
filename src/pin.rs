use std::collections::HashMap;

use cached::proc_macro::cached;
use cached::Return;

use serenity::client::Context;
use serenity::http::client::Http;
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, GuildId, MessageId};

use serenity::framework::standard::{
    Args,
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
#[commands(pin)]
struct Pin;


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
