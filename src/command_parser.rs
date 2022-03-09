use regex::Regex;
use std::error::Error;
use serenity::{model::channel::Message, prelude::*};

use super::{minecraft, music, utilities};

pub async fn parse(msg: &Message, ctx: &Context, thread: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let stuff: Vec<&str> = Regex::new(r" ").unwrap().split(&thread).collect();
    let cmd = stuff[0];
    let args = if stuff.len() == 1 { "".into() } else { stuff[1].into() };

    match cmd {
        "bond" | "chess" | "compile" | "find" | "gato" | "wa" | "help" => { utilities::exec(cmd, msg, ctx, args).await? }

        "ms" | "mcskin" | "achieve"  => { minecraft::exec(cmd, msg, ctx, args).await? }

        "play" | "p" | "pause" | "resume" | "replay" | "skip" | "s" | "stop" | "songinfo" | "queue" | "leave" | "volume" | "vol" => { music::exec(cmd, msg, ctx, args).await? }
        
        _ => {}
    }

    Ok(())
}