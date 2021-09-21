use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};
use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::model::channel::Message;

#[group]
#[commands(shibe)]
struct Shibe;

#[command]
async fn shibe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = match args.single::<String>(){
        Ok(x) => x,
        _ => {
            msg.channel_id.say(&ctx.http, "must be polite >:(").await?;
            String::from("Error")
        }
    };
    let _second = args.single::<String>()?;

    if first == "error" {
        msg.reply(&ctx.http, "no >:(").await.unwrap();
        return Ok(());
    }

    let mut resp = reqwest::get("https://shibe.online/api/shibes?count=1&urls=true&httpsUrls=true")
        .await?
        .json::<Vec<String>>()
        .await?;
    msg.channel_id.say(&ctx.http, resp.pop().unwrap()).await.unwrap();
    Ok(())
}