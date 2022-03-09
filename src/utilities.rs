use std::{env, error::Error};
use serenity::{model::{channel::Message, invite::InviteTargetType}, prelude::*, utils::Colour};
use regex::Regex;
use genius_rs::Genius;

use super::{helper::{langmap, bondapp}, structs::{CompilerPost, CompilerResponse, Gato, Wa, Starwa}};

pub async fn exec(cmd: &str, msg: &Message, ctx: &Context, args: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rn = chrono::Utc::now();
    let embed_color = Colour::from_rgb(242, 82, 120);

    match cmd {
        "help" => {
            msg.channel_id.send_message(ctx, |m| {
                m.content("comg");
                m.embed(|e| {
                    e.title("Sakura");
                    e.description("Rust powered naruto gal");
                    e.thumbnail("https://i.imgur.com/cLPKmFQ.png");
                    e.color(embed_color);
                    e.field("Current prefix: `bb`", "----------------------------", false);
                    e.field("Commands:", "
                    `bond` - Bonding activities
                    `chess` - Info about chess
                    `help` - Show this panel
                    `ms` - Show a minecraft server's status
                    `mcskin` - Get a minecraft player's skin
                    `achieve` - Achievement got!
                    `gato` - Gato pic
                    `wa` - degeneralte
                    `compile` - Compile ur spaghetti", false);
                    e.footer(|f| f.text("oki, have fun"));
                    e.timestamp(&rn);
                    e
                });
                m
            }).await?;
        }

        "chess" => { msg.reply(ctx, "♟ Use `bb bond chess` to play Chess In The Park!").await?; }

        "compile" => {
            let rex = Regex::new(r"((?s)\w+) ```\w+((?s).*?)```").unwrap(); // workable
            let stuff = rex.captures(&args).unwrap();
            let lang = stuff.get(1).unwrap().as_str();
            let code = stuff.get(2).unwrap().as_str().into();
            let lang_version = langmap(lang);

            if lang_version == 1 {
                msg.channel_id.say(ctx, "📜 Ight use valid syntax: `c | cpp | csharp | objc | java | nodejs | lua | rust | python3 | ruby | brainfuck | go | swift | perl | php | sql | clojure | coffeescript | elixir | lolcode | kotlin | groovy | octave`
                __**Example:**__ bbcompile rust \\`\\`\\`rust
                fn main() {
                    println!(\"bb naoh\")
                }
                \\`\\`\\`").await?;
            } else {
                let program = CompilerPost {
                    script: code,
                    language: lang.into(),
                    versionIndex: lang_version,
                    clientId: env::var("JD_CLI_ID")?,
                    clientSecret: env::var("JD_CLI_SECRET")?
                };

                let data = reqwest::Client::new().post("https://api.jdoodle.com/v1/execute")
                    .json(&program)
                    .send().await?
                    .json::<CompilerResponse>().await?;

                msg.channel_id.send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title("📜 Output:");
                        e.description(format!("```{}```", data.output));
                        e.color(embed_color);
                        e.footer(|f| f.text(format!("CPU time: {}ms", data.cpuTime)));
                        e.timestamp(&rn);
                        e
                    });
                    m
                }).await?;
            }
        }

        "bond" => {
            let guild = msg.guild(&ctx.cache).await.unwrap();
            let channel_id = match guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id) {
                Some(id) => id,
                None => {
                    msg.reply(ctx, "💞 Join a voice channel to start bonding!").await?;
                    return Ok(());
                }
            };

            let activity = bondapp(&args);
            if activity == 0 {
                msg.reply(ctx, "💞 Some activities I found: `youtube | poker | betrayal | fishing | chess | lettertile | wordsnack | doodlecrew | awkword | spellcast | checkers | puttparty | sketchyartist`").await?;
                return Ok(());
            }

            let channel_name = channel_id.name(&ctx.cache).await.unwrap();

            let invite = channel_id.create_invite(&ctx, |i| {
                i.max_age(86400);
                i.max_uses(0);
                i.temporary(false);
                i.target_application_id(activity.into());
                i.target_type(InviteTargetType::EmmbeddedApplication);
                i
            }).await?;

            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(format!("💞 {} bonding time", &guild.name));
                    e.description(format!("🔸 Activity: {}", &args));
                    e.thumbnail(guild.icon_url().unwrap());
                    e.color(embed_color);
                    e.field(format!("Join {:?}", channel_name), invite.url(), false);
                    e.timestamp(&rn);
                    e
                });
                m
            }).await?;
        }

        "find" => {
            let genius = Genius::new(env::var("GENIUS_TOKEN")?);
            let res = genius.search(&args).await.unwrap();
            let data = &res[0].result;
            let artist = &data.primary_artist;

            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(data.title.as_str());
                    e.description(format!("By [{}]({})", artist.name, artist.url));
                    e.color(embed_color);
                    e.field("More about song:", data.url.as_str(), false);
                    e.thumbnail(artist.header_image_url.as_str());
                    e.image(data.song_art_image_url.as_str());
                    e
                });
                m
            }).await?;
        }

        "wa" => {
            if !msg.channel(&ctx.cache).await.unwrap().is_nsfw() {
                msg.channel_id.say(ctx, "🌸 Oui, nsfw channel only!").await?;
                return Ok(());
            }

            let img = if args == "" {
                let url = "https://api.waifu.pics/sfw/waifu";
                let body = reqwest::get(url).await?.json::<Wa>().await?;
                body.url
            } else {
                let url = format!("{}{}", env::var("SW")?, args);
                let body = reqwest::get(url).await?.json::<Starwa>().await?;
                body.url
            };

            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("wa");
                    e.color(embed_color);
                    e.image(img);
                    e
                });
                m
            }).await?;
        }

        "gato" => {
            let url = "https://aws.random.cat/meow?ref=apilist.fun";
            let body = reqwest::get(url).await?.json::<Gato>().await?;

            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("gatex");
                    e.color(embed_color);
                    e.image(body.file);
                    e.timestamp(&rn);
                    e
                });
                m
            }).await?;
        }
        _ => {}
    }

    Ok(())
}