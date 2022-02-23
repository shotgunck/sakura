use actix_web::{App, get, HttpServer, Responder};
use dotenv::dotenv;
use std::{env, error::Error};
use regex::Regex;
use serenity::{async_trait, model::{channel::Message, gateway::Ready}, prelude::*};

mod utilities;
mod structs;
mod minecraft;

struct Handler;

#[get("/")]
async fn comg() -> impl Responder { "sakura さくら v4" }

#[actix_web::main]
async fn live() -> Result<(), Box<dyn Error>> {
    HttpServer::new(|| {
        App::new().service(comg)
    })
    .bind("0.0.0.0:8088").unwrap()
    .run()
    .await?;

    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            let compound_rex = Regex::new(r" && ").unwrap();
            let prefix_rex = Regex::new(r"bb |bb").unwrap();
            let threads: Vec<&str> = compound_rex.split(&msg.content).collect();
            
            if prefix_rex.is_match(&msg.content) {
                for thread in threads.iter() {
                    utilities::init(&msg, &ctx, prefix_rex.replace(&thread.to_ascii_lowercase(), "").into()).await.expect("can't exec cmd");
                }
            }
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("sakura さくら v4 op");
        live().expect("uhh can't start actix")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    let token = env::var("BOT_TOKEN")?;
    let mut client = Client::builder(&token).event_handler(Handler).await.expect("could not make client");

    if let Err(why) = client.start().await {
        println!("startup failure {:?}", why)
    }
    Ok(())
}