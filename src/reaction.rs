use std::convert::TryFrom;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::channel::ReactionType;
use serenity::model::id::{ChannelId};

use serenity::prelude::Mentionable;
use serenity::utils::Colour;

use std::env;

pub struct Handler;
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