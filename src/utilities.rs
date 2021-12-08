use std::{env, error::Error, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use twilight_http::Client as HttpClient;
use twilight_model::{id::ChannelId as ChannelId, datetime::Timestamp};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};

use super::structs;

pub async fn bond(http: Arc<HttpClient>, channel_id: ChannelId) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("https://discord.com/api/v8/channels/{}/invites", channel_id);
    let client = reqwest::Client::builder().build()?;
    
    let rn = Timestamp::from_secs(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().try_into()?)?;

    let data = structs::BondPost {
        max_age: 86400,
        max_uses: 0,
        target_application_id: "880218394199220334".to_string(),
        target_type: 2,
        temporary: false,
        validate: None
    };
    let invite = client.post(url)
        .json(&data)
        .header("Authorization", format!("Bot {}", env::var("BOT_TOKEN")?))
        .header("Content-Type", "application/json")
        .send().await?
        .json::<structs::BondResponse>().await?;
    
    let bondinvite = EmbedBuilder::new()
        .title(format!("💞 {}'s bonding time!", invite.guild.name))
        .field(EmbedFieldBuilder::new("Click to join:", format!("https://discord.gg/{}", invite.code)))
        .color(0xff_b8_b8)
        .timestamp(rn)
        .build();
    http.create_message(channel_id).embeds(&[bondinvite?])?.exec().await?;

    Ok(())
}

pub async fn help(http: Arc<HttpClient>, channel_id: ChannelId) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rn = Timestamp::from_secs(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().try_into()?)?;

    let helpembed = EmbedBuilder::new()
        .title("Sakura")
        .description("Rust powered naruto gal")
        .thumbnail(ImageSource::url("https://i.imgur.com/cLPKmFQ.png")?)
        .color(0xff_b8_b8)
        .field(EmbedFieldBuilder::new("Current prefix: `bb`", "----------------------------").inline())
        .field(EmbedFieldBuilder::new("Commands:", "? - see if im on
            help - helplist
            ms - show a mc server's status
            mcskin - mc player skin get
            gato - gato helicoper
            wa - degeneralte
        "))
        .footer(EmbedFooterBuilder::new("oki, have fun"))
        .timestamp(rn)
        .build();
    http.create_message(channel_id).embeds(&[helpembed?])?.exec().await?;

    Ok(())
}

pub async fn gato(http: Arc<HttpClient>, channel_id: ChannelId) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "https://aws.random.cat/meow?ref=apilist.fun";
    let body = reqwest::get(url).await?.json::<structs::Gato>().await?;

    let gatopic = EmbedBuilder::new()
        .title("gato")
        .color(0xff_b8_b8)
        .image(ImageSource::url(body.file)?)
        .build();
    http.create_message(channel_id).embeds(&[gatopic?])?.exec().await?;

    Ok(())
}

pub async fn wa(http: Arc<HttpClient>, channel_id: ChannelId) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "https://api.waifu.pics/sfw/waifu";
    let body = reqwest::get(url).await?.json::<structs::Wa>().await?;

    let wapic = EmbedBuilder::new()
        .title("wa?!")
        .color(0xff_b8_b8)
        .image(ImageSource::url(body.url)?)
        .build();
    http.create_message(channel_id).embeds(&[wapic?])?.exec().await?;

    Ok(())
}