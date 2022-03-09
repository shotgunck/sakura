use actix_web::{App, get, HttpServer, Responder};
use dotenv::dotenv;
use std::{env, error::Error, sync::Arc};
use regex::Regex;
use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use serenity::{async_trait, http::Http, model::{channel::Message, gateway::{Ready, Activity}, user::OnlineStatus}, prelude::*};
use songbird::SerenityInit;

mod command_parser;
mod helper;
mod minecraft;
mod music;
mod structs;
mod utilities;

use structs::{Handler, Lavalink, LavalinkHandler, SerenityContext};
use command_parser::parse;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}

impl TypeMapKey for SerenityContext {
    type Value = (Message, Arc<Http>);
}

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
    async fn track_start(&self, lava_client: LavalinkClient, event: TrackStart) {
        let node = lava_client.nodes().await;
        let guild_node = node.get(&event.guild_id.0).unwrap();
        let data = &guild_node.data.write();
        let (msg, http) = data.get::<SerenityContext>().unwrap();

        let track = guild_node.now_playing.as_ref().unwrap().track.info.as_ref().unwrap();
        msg.channel_id.say(http, format!("🎶 Now playing: **{}**", track.title)).await.expect("sa");
    }
}

#[get("/")]
async fn comg() -> impl Responder { "sakura さくら v5 webserver" }

#[actix_web::main]
async fn live() -> Result<(), Box<dyn Error>> {
    HttpServer::new(|| App::new().service(comg))
    .bind(format!("0.0.0.0:{}", env::var("PORT")?)).unwrap()
    .run()
    .await?;

    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {return}

        let compound_rex = Regex::new(r" && ").unwrap();
        let prefix_rex = Regex::new(r"bb |bb").unwrap();
        let threads: Vec<&str> = compound_rex.split(&msg.content).collect();
            
        if prefix_rex.is_match(&msg.content) {
            for thread in threads.iter() {
                parse(&msg, &ctx, prefix_rex.replace(&thread.to_ascii_lowercase(), "").into()).await.expect("can't exec cmd");
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        println!("sakura さくら v5 re");

        ctx.set_presence(Some(Activity::playing("with hanako")), OnlineStatus::DoNotDisturb).await;
        live().expect("actix failure");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    let token = env::var("BOT_TOKEN")?;
    let mut client = Client::builder(&token).event_handler(Handler).register_songbird().await.expect("could not make client");

    let http = Http::new_with_token(&token);

    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("http error {:?}", why),
    };

    let lava_client = LavalinkClient::builder(bot_id)
        .set_host(env::var("LAVALINK_HOST")?)
        .set_password(env::var("LAVALINK_PASS")?)
        .set_port(8880)
        .build(LavalinkHandler)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<Lavalink>(lava_client);
    }

    client.start().await?;

    Ok(())
}