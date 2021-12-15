use serde::{Serialize, Deserialize};
use serenity::model::channel::Message;

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
pub struct Wa { //laffable
    pub url: String
}

#[derive(Serialize, Debug)]
pub struct BondPost {
    pub max_age: u32,
    pub max_uses: u32,
    pub target_application_id: String,
    pub target_type: u32,
    pub temporary: bool,
    pub validate: Option<bool>
}

#[derive(Deserialize, Debug)]
pub struct BondGuildInfo {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct BondResponse {
    pub code: String,
    pub guild: BondGuildInfo
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
    pub output: String
}

pub struct Lavalink;
