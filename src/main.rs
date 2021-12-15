#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use dotenv::dotenv;
use std::{env, error::Error};
use regex::Regex;
use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use serenity::{async_trait, http::Http, model::{channel::Message, gateway::Ready}, prelude::*};
use songbird::SerenityInit;

mod utilities;
mod structs;
mod minecraft;
mod music;

impl TypeMapKey for structs::Lavalink {
    type Value = LavalinkClient;
}

struct Handler;
struct LavalinkHandler;

#[get("/")]
fn comg() -> &'static str { "sakura v2" }

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            let compound_rex = Regex::new(r" && ").unwrap();
            let prefix_rex = Regex::new(r"bb |bb").unwrap();
            let threads: Vec<&str> = compound_rex.split(&msg.content).collect();
            
            if prefix_rex.is_match(&msg.content) {
                for thread in threads.iter() {
                    utilities::init(&msg, &ctx, prefix_rex.replace(&thread.to_ascii_lowercase(), "").into()).await.expect("can't exec cmd tbh");
                }
            }
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("sakura is on");
        rocket::ignite().mount("/", routes![comg]).launch();
    }
}

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
    async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
        println!("playin in guild {}", event.guild_id);
    }
    async fn track_finish(&self, _client: LavalinkClient, _event: TrackFinish) {
        println!("finished");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN")?;

    let http = Http::new_with_token(&token);

    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("couldn't get info cus {:?}", why),
    };

    let mut client = Client::builder(&token).event_handler(Handler).register_songbird().await.expect("could not make client");

    let lava_client = LavalinkClient::builder(bot_id).set_host(env::var("LAVALINK_HOST")?).set_port(2333).set_password(env::var("LAVALINK_PASS")?)
        .build(LavalinkHandler)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<structs::Lavalink>(lava_client);
    }
    
    if let Err(why) = client.start().await {
        println!("startup failure {:?}", why);
    }
    Ok(())
}