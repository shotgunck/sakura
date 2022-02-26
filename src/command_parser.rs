use regex::Regex;
use std::error::Error;
use serenity::{model::channel::Message, prelude::*};

use super::{minecraft, music, utilities};

pub async fn parse(msg: &Message, ctx: &Context, thread: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let stuff: Vec<&str> = Regex::new(r"\s").unwrap().split(&thread).collect();
    let cmd = stuff[0];
    let args = Regex::new(&format!(r"{} ", &cmd)).unwrap().replace(&thread, "").into();

    match cmd {
        "help" | "chess" | "bond" | "compile" | "wa" | "gato" => { utilities::exec(cmd, msg, ctx, args).await? },
        
        "ms" | "mcskin" | "achieve"  => { minecraft::exec(cmd, msg, ctx, args).await? },
        
        "play" | "p" | "pause" | "resume" | "replay" | "skip" | "s" | "stop" | "songinfo" | "queue" | "leave" | "volume" | "vol" => { music::exec(cmd, msg, ctx, args).await? }
        
        _ => {}
    }

    Ok(())
}