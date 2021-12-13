use std::{error::Error, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use twilight_http::Client as HttpClient;
use twilight_model::{id::ChannelId as ChannelId, datetime::Timestamp};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, ImageSource};

use super::structs;

pub async fn ms(http: Arc<HttpClient>, channel_id: ChannelId, arguments: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("https://mcapi.xdefcon.com/server/{}/full/json", arguments);
    let body = reqwest::get(&url).await?.json::<structs::MSResponse>().await?;

    let rn = Timestamp::from_secs(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().try_into()?)?;
    let msembed = if body.serverStatus == "online" {
        EmbedBuilder::new()
            .title(format!("🟢 {} is online", arguments))
            .description(body.motd.text)
            .thumbnail(ImageSource::url(format!("https://eu.mc-api.net/v3/server/favicon/{}", arguments))?)
            .color(0xff_b8_b8)
            .field(EmbedFieldBuilder::new("🗨 Info:", format!("--------------------------------
            
            **Version:** {}

            **Players in game:** {}/{}

            **Ping:** {}ms

            --------------------------------
            🔸 This is a cached result. Please check again in few minutes!
            ", body.version, body.players, body.maxplayers, body.ping)))
            .timestamp(rn)
            .build()
    } else {
        EmbedBuilder::new()
            .title(format!("🔴 {} is offline", arguments))
            .description("Try again later k")
            .color(0xff_b8_b8)
            .timestamp(rn)
            .build()
    };

    http.create_message(channel_id).embeds(&[msembed?])?.exec().await?;

    Ok(())
}

pub async fn mcskin(http: Arc<HttpClient>, channel_id: ChannelId, arguments: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if arguments == "" {
        http.create_message(channel_id).content("👨‍💻 Provide a player name plsss")?.exec().await?;
    } else {
        http.create_message(channel_id).embeds(&[EmbedBuilder::new()
            .title(&arguments)
            .color(0xff_b8_b8)
            .image(ImageSource::url(format!("https://minotar.net/armor/body/{}/150.png", arguments))?)
            .build()?])?.exec().await?;
    }
    Ok(())
}

pub async fn achieve(http: Arc<HttpClient>, channel_id: ChannelId, arguments: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    http.create_message(channel_id).embeds(&[EmbedBuilder::new()
        .title("​")
        .color(0xff_b8_b8)
        .image(ImageSource::url(format!("https://minecraft-api.com/api/achivements/cake/achievement..got/{}", arguments))?)
        .build()?
    ])?.exec().await?;

    Ok(())
}