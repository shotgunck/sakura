use std::{error::Error, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use twilight_http::Client as HttpClient;
use twilight_model::{id::ChannelId as ChannelId, datetime::Timestamp};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, ImageSource};

use super::structs;

pub async fn ms(http: Arc<HttpClient>, channel_id: ChannelId, arguments: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("https://mcapi.xdefcon.com/server/{}/full/json", arguments);
    let mut body = reqwest::get(&url).await?.json::<structs::MSResponse>().await?;

    image::load_from_memory(&base64::decode(body.icon.split_off(22)).unwrap())?.save("servericon.png")?;

    let rn = Timestamp::from_secs(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().try_into()?)?;
    let msembed = if body.serverStatus == "online" {
        EmbedBuilder::new()
            .title(format!("🟢 {} is online", arguments))
            .description(body.motd.text)
            .thumbnail(ImageSource::attachment("servericon.png")?)
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
    }

    let skinembed = EmbedBuilder::new()
        .title(&arguments)
        .color(0xff_b8_b8)
        .image(ImageSource::url(format!("https://minotar.net/armor/body/{}/150.png", arguments))?)
        .build();
    http.create_message(channel_id).embeds(&[skinembed?])?.exec().await?;

    Ok(())
}