use regex::Regex;
use std::collections::HashMap;
use std::{env, error::Error};
use serenity::{model::channel::Message, prelude::*, utils::Colour};

use super::{structs, minecraft};

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

pub async fn init(msg: &Message, ctx: &Context, thread: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let stuff: Vec<&str> = Regex::new(r"\s").unwrap().split(&thread).collect();//.captures(&thread).unwrap().get(1).unwrap().as_str();
    let cmd = stuff[0];
    let args = Regex::new(&format!(r"{} ", &cmd)).unwrap().replace(&thread, "");
    let rn = chrono::Utc::now();

    match cmd {    
        "help" => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.content("comg");
                m.embed(|e| {
                    e.title("Sakura");
                    e.description("Rust powered naruto gal");
                    e.thumbnail("https://i.imgur.com/cLPKmFQ.png");
                    e.color(Colour::from_rgb(255, 184, 184));
                    e.field("Current prefix: `bb`", "----------------------------", false);
                    e.field("Commands:", "
                    help    - show commands
                    ms      - show a minecraft server's status
                    mcskin  - get a minecraft player's skin
                    gato    - gato helicoper
                    wa      - degeneralte
                    compile - compile ur spaghetti", false);
                    e.footer(|f| {
                        f.text("oki, have fun"); f
                    });
                    e.timestamp(&rn);
                    e
                });
                m
            }).await?;
        }
        
        "compile" => {
            let rex = Regex::new(r"((?s)\w+) ```\w+((?s).*?)```").unwrap(); // workable
            let stuff = rex.captures(&args).unwrap();
            let lang = stuff.get(1).unwrap().as_str();
            let code = stuff.get(2).unwrap().as_str().into();

            if !langmap().contains_key(lang) {
                msg.channel_id.say(&ctx.http, "📜 Ight use valid syntax: `c | cpp | csharp | objc | java | nodejs | lua | rust | python3 | ruby | brainfuck | go | swift | perl | php | sql | clojure | coffeescript | elixir | lolcode | kotlin | groovy | octave`
        __**Example:**__
        bbcompile rust \\`\\`\\`rust
        fn main() { println!(\"workable code clentaminator\"); }
        \\`\\`\\`
        ").await?;
            } else {
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

                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("📜 Output:");
                        e.description(output.output);
                        e.color(Colour::from_rgb(255, 184, 184));
                        e.timestamp(&rn);
                        e
                    });
                    m
                }).await?;
            }
        }

        "bond" => {
            let url = format!("https://discord.com/api/v8/channels/{}/invites", msg.channel_id);
    
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
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("💞 {}'s bonding time!", invite.guild.name));
                    e.field("Click to join: ", format!("https://discord.gg/{}", invite.code), false);
                    e.color(Colour::from_rgb(255, 184, 184));
                    e.timestamp(&rn);
                    e
                });
                m
            }).await?;
        }

        "wa" => {
            let url = "https://api.waifu.pics/sfw/waifu";
            let body = reqwest::get(url).await?.json::<structs::Wa>().await?;
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("wa,,");
                    e.color(Colour::from_rgb(255, 184, 184));
                    e.image(body.url);
                    e
                });
                m
            }).await?;
        }

        "gato" => {
            let url = "https://aws.random.cat/meow?ref=apilist.fun";
            let body = reqwest::get(url).await?.json::<structs::Gato>().await?;
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("gatex");
                    e.color(Colour::from_rgb(255, 184, 184));
                    e.image(body.file);
                    e.timestamp(&rn);
                    e
                });
                m
            }).await?;
        }

        "ms" => { minecraft::ms(msg, ctx, args.into()).await?; }
        "mcskin" => { minecraft::mcskin(msg, ctx, args.into()).await?; }
        "achieve" => { minecraft::achieve(msg, ctx, args.into()).await?; }

        _ => {}
    }

    Ok(())
}