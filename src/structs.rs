#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct Motd {
    pub text: String
}

#[derive(Deserialize, Debug)]
pub struct MSResponse {
    pub serverStatus: String,
    pub motd: Motd,
    pub ping: u32,
    pub players: u32,
    pub maxplayers: u32,
    pub version: String,
    pub icon: String
}

#[derive(Deserialize, Debug)]
pub struct Gato {
    pub file: String
}

#[derive(Deserialize, Debug)]
pub struct Wa {
    pub url: String
}

#[derive(Deserialize, Debug)]
pub struct Starwa {
    pub url: String
}

#[derive(Serialize, Debug)]
pub struct CompilerPost {
    pub script: String,
    pub language: String,
    pub versionIndex: u8,
    pub clientId: String,
    pub clientSecret: String
}

#[derive(Deserialize, Debug)]
pub struct CompilerResponse {
    pub output: String,
    pub cpuTime: String
}

pub struct Handler;
pub struct Lavalink;
pub struct  LavalinkHandler;
pub struct SerenityContext;