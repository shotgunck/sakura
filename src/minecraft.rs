use std::error::Error;
use serenity::{model::{channel::Message}, prelude::*, utils::Colour};

use super::structs;

pub async fn ms(msg: &Message, ctx: &Context, arg: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("https://mcapi.xdefcon.com/server/{}/full/json", arg);
    let body = reqwest::get(&url).await?.json::<structs::MSResponse>().await?;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| if body.serverStatus == "online" {
            e.title(format!("🟢 {} is online", arg));
            e.description(body.motd.text);
            e.color(Colour::from_rgb(255, 184, 184));
            e.thumbnail(format!("https://eu.mc-api.net/v3/server/favicon/{}", arg));
            e.field("📜 Info:", format!("--------------------------------
    
            **Version:** {}

            **Players in game:** {}/{}

            **Ping:** {}ms

            --------------------------------
            🔸 This is a cached result. Please check again in few minutes!
            ", body.version, body.players, body.maxplayers, body.ping), false);
            e.timestamp(&chrono::Utc::now());
            e
        } else {
            e.title(format!("🔴 {} is offline", arg));
            e.description("Try again in 5 minutes!");
            e
        })
    }).await?;

    Ok(())
}


pub async fn mcskin(msg: &Message, ctx: &Context, arguments: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if arguments == "" {
        msg.channel_id.say(&ctx.http, "👨‍💻 Provide a player name plsss").await?;
    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&arguments);
                e.color(Colour::from_rgb(255, 184, 184));
                e.image(format!("https://minotar.net/armor/body/{}/150.png", arguments));
                e
            });
            m
        }).await?;
    }
    Ok(())
}

pub async fn achieve(msg: &Message, ctx: &Context, arguments: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&arguments);
            e.color(Colour::from_rgb(255, 184, 184));
            e.image(format!("https://minecraft-api.com/api/achivements/cake/achievement..got/{}", arguments));
            e
        });
        m
    }).await?;

    Ok(())
}
