use regex::Regex;
use std::{collections::HashMap, env, error::Error, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use twilight_http::Client as HttpClient;
use twilight_model::{id::ChannelId as ChannelId, datetime::Timestamp};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};

use super::structs;

fn langmap() -> HashMap<String, u8> {
    let mut langver = HashMap::<String, u8>::new();
    langver.insert("java".into(), 3); langver.insert("c".into(), 4); langver.insert("cpp".into(), 4);
    langver.insert("php".into(), 3); langver.insert("perl".into(), 3); langver.insert("python3".into(), 3);
    langver.insert("ruby".into(), 3); langver.insert("go".into(), 3); langver.insert("clojure".into(), 2);
    langver.insert("sql".into(), 3); langver.insert("csharp".into(), 3); langver.insert("objc".into(), 3);
    langver.insert("swift".into(), 3); langver.insert("brainfuck".into(), 0); langver.insert("lua".into(), 2);
    langver.insert("rust".into(), 3); langver.insert("nodejs".into(), 3); langver.insert("coffeescript".into(), 3);
    langver.insert("elixir".into(), 3); langver.insert("lolcode".into(), 0); langver.insert("kotlin".into(), 2);
    langver.insert("groovy".into(), 3); langver.insert("octave".into(), 3);

    langver
}

pub async fn bond(http: Arc<HttpClient>, channel_id: ChannelId) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("https://discord.com/api/v8/channels/{}/invites", channel_id);
    
    let rn = Timestamp::from_secs(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().try_into()?)?;

    let data = structs::BondPost {
        max_age: 86400,
        max_uses: 0,
        target_application_id: "880218394199220334".into(),
        target_type: 2,
        temporary: false,
        validate: None
    };
    let invite = reqwest::Client::new().post(url)
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
            compile - compile ur spaghetti
        "))
        .footer(EmbedFooterBuilder::new("oki, have fun"))
        .timestamp(rn)
        .build();
    http.create_message(channel_id).embeds(&[helpembed?])?.exec().await?;

    Ok(())
}

pub async fn compile(http: Arc<HttpClient>, channel_id: ChannelId, arguments: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rex = Regex::new(r"((?s)\w+) ```\w+((?s).*?)```").unwrap(); //trash regex usr
    let stuff = rex.captures(&arguments).unwrap();
    let lang = stuff.get(1).unwrap().as_str();
    let code = stuff.get(2).unwrap().as_str().into();

    if !langmap().contains_key(lang) {
        http.create_message(channel_id).content("📜 Ight use valid syntax: `c | cpp | csharp | objc | java | nodejs | lua | rust | python3 | ruby | brainfuck | go | swift | perl | php | sql | clojure | coffeescript | elixir | lolcode | kotlin | groovy | octave`
__**Example:**__
bbcompile rust \\`\\`\\`rust
fn main() { println!(\"workable code clentaminator\"); }
\\`\\`\\`
")?.exec().await?;
    }

    let program = structs::CompilerPost {
        script: code,
        language: lang.into(),
        versionIndex: *langmap().get(lang).unwrap(),
        clientId: env::var("JD_CLI_ID")?,
        clientSecret: env::var("JD_CLI_SECRET")?
    };

    let url = "https://api.jdoodle.com/v1/execute";
    let output = reqwest::Client::new().post(url)
        .json(&program)
        .send().await?
        .json::<structs::CompilerResponse>().await?;

    let rn = Timestamp::from_secs(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().try_into()?)?;

    http.create_message(channel_id).embeds(&[EmbedBuilder::new()
        .title("📜 Output:")
        .description(output.output)
        .timestamp(rn)
        .build()?])?.exec().await?;

    Ok(())
}

pub async fn gato(http: Arc<HttpClient>, channel_id: ChannelId) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "https://aws.random.cat/meow?ref=apilist.fun";
    let body = reqwest::get(url).await?.json::<structs::Gato>().await?;

    http.create_message(channel_id).embeds(&[EmbedBuilder::new()
        .title("gato")
        .color(0xff_b8_b8)
        .image(ImageSource::url(body.file)?)
        .build()?])?.exec().await?;

    Ok(())
}

pub async fn wa(http: Arc<HttpClient>, channel_id: ChannelId) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "https://api.waifu.pics/sfw/waifu";
    let body = reqwest::get(url).await?.json::<structs::Wa>().await?;

    http.create_message(channel_id).embeds(&[EmbedBuilder::new()
        .title("wa?!")
        .color(0xff_b8_b8)
        .image(ImageSource::url(body.url)?)
        .build()?])?.exec().await?;

    Ok(())
}