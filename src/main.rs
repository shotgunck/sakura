#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use dotenv::dotenv;
use futures_util::StreamExt;
use std::{env, error::Error, sync::Arc};

use regex::Regex;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::Cluster, Event, Intents};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::{payload::outgoing::update_presence::UpdatePresencePayload, presence::{ActivityType, MinimalActivity, Status}};
use twilight_command_parser::{Command, CommandParserConfig, Parser};

mod utilities;
mod minecraft;
//mod music;
mod structs;

#[get("/")]
fn hello() -> &'static str { "sakuri" }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN")?;
    let cache = InMemoryCache::builder().resource_types(ResourceType::MESSAGE).build();
    
    let mut config = CommandParserConfig::new(); config.add_prefix("bb");
    config.add_command("?", false);
    config.add_command("help", false);
    config.add_command("ms", false);
    config.add_command("mcskin", false);
    config.add_command("gato", false);
    config.add_command("wa", false);
    config.add_command("bond", false);
    config.add_command("compile", false);
    config.add_command("achieve", false);

    let parser = Parser::new(config);

    let http = Arc::new(HttpClient::new(token.clone()));

    let (cluster, mut events) = Cluster::builder(token.clone(), Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES)
        .presence(UpdatePresencePayload::new(vec![
        MinimalActivity {
            kind: ActivityType::Listening,
            name: "bb?".into(),
            url: None
        }.into(),
        ], false, None, Status::DoNotDisturb)?)
        .build().await?;
    cluster.up().await;

    while let Some((_, event)) = events.next().await {
        cache.update(&event);
        tokio::spawn(handle_event(event, Arc::clone(&http), parser.clone()));
    }
    
    Ok(())
}

async fn handle_event(event: Event, movedhttp: Arc<HttpClient>, parser: Parser<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) => {
            let channel_id = msg.channel_id;
            let rex = Regex::new(r" && ").unwrap();
            let threads: Vec<&str> = rex.split(&msg.content).collect();

            for thread in threads.iter() {
                let http = Arc::clone(&movedhttp);
                match parser.parse(thread) {
                    Some(Command { name: "?", .. }) => {
                        http.create_message(channel_id).content("hi babe")?.exec().await?;
                    }
                    Some(Command { name: "help", .. }) => {
                        utilities::help(http, channel_id).await?;
                    }
                    Some(Command { name: "mcskin", arguments, ..}) => {
                        minecraft::mcskin(http, channel_id, arguments.as_str().into()).await?;
                    }
                    Some(Command { name: "ms", arguments, ..}) => {
                        minecraft::ms(http, channel_id, arguments.as_str().into()).await?;
                    }
                    Some(Command { name: "gato", ..}) => {
                        utilities::gato(http, channel_id).await?;
                    }
                    Some(Command { name: "wa", ..}) => {
                        utilities::wa(http, channel_id).await?;
                    }
                    Some(Command { name: "bond", ..}) => {
                        utilities::bond(http, channel_id).await?;
                    }
                    Some(Command { name: "compile", arguments, ..}) => {
                        utilities::compile(http, channel_id, arguments.as_str().into()).await?;
                    }
                    Some(Command { name: "achieve", arguments, ..}) => {
                        minecraft::achieve(http, channel_id, arguments.as_str().into()).await?;
                    }

                    Some(_) => {}
                    None => {}
                }
            }
        }

        Event::ShardConnected(_) => {
            println!("sakura runnin");
            rocket::ignite().mount("/", routes![hello]).launch();
        }
        _ => {}
    }

    Ok(())
}