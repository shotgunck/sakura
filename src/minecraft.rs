use std::error::Error;
use serenity::{model::channel::Message, prelude::*, utils::Colour};

use super::structs::MSResponse;

pub async fn exec(cmd: &str, msg: &Message, ctx: &Context, args: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cmd {
        "ms" => {
            let url = format!("https://mcapi.xdefcon.com/server/{}/full/json", args);
            let body = reqwest::get(&url).await?.json::<MSResponse>().await?;

            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| if body.serverStatus == "online" {
                    e.title(format!("🟢 {} is online", args));
                    e.description(body.motd.text);
                    e.color(Colour::from_rgb(255, 184, 184));
                    e.thumbnail(format!("https://eu.mc-api.net/v3/server/favicon/{}", args));
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
                    e.title(format!("🔴 {} is offline", args));
                    e.description("Try again in 5 minutes!");
                    e
                })
            }).await?;
        },

        "mcskin" => {
            if args == "" {
                msg.channel_id.say(ctx, "👨‍💻 Provide a player name pls").await?;
                return Ok(())
            }
            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(&args);
                    e.color(Colour::from_rgb(255, 184, 184));
                    e.image(format!("https://minotar.net/armor/body/{}/150.png", args));
                    e
                });
                m
            }).await?;
        },

        "achieve" => {
            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(&args);
                    e.color(Colour::from_rgb(255, 184, 184));
                    e.image(format!("https://minecraft-api.com/api/achivements/cake/achievement..got/{}", args.replace(" ", "..")));
                    e
                });
                m
            }).await?;
        }
        _ => {}
    }

    Ok(())
}