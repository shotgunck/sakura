use std::error::Error;
use songbird;
use serenity::{model::{channel::Message}, prelude::*};

use super::structs::Lavalink;

pub async fn play(msg: &Message, ctx: &Context, args: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "Join a voice channel first ke").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(&ctx).await.expect("something bad").clone();

    let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

    match handler {
        Ok(connection_info) => {
            let data = ctx.data.read().await;
            let lava_client = data.get::<Lavalink>().unwrap().clone();
            lava_client.create_session_with_songbird(&connection_info).await?;
        }
        Err(why) => {
            msg.channel_id.say(&ctx.http, format!("Nopr can't join channel: {}", why)).await?;
        },
    }

    let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();

    let query_info = lava_client.auto_search_tracks(&args).await?;

    if query_info.tracks.is_empty() {
        msg.channel_id.say(&ctx, "🥟 Can't find song with given name").await?;
        return Ok(());
    } else if query_info.tracks.len() >= 1 {
        lava_client.play(guild_id, query_info.tracks[0].clone()).start().await?;
        msg.channel_id.say(&ctx.http, format!("Now playin: **{}**", query_info.tracks[0].info.as_ref().unwrap().title)).await?;
    } else {
        lava_client.play(guild_id, query_info.tracks[0].clone()).queue().await?;
        msg.channel_id.say(&ctx.http, format!("Added to queue: **{}**", query_info.tracks[0].info.as_ref().unwrap().title)).await?;
    }


    Ok(())
}
