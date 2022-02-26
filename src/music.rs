use std::error::Error;
use songbird::get;
use serenity::{model::{channel::{Message, ReactionType}, id::GuildId}, prelude::*, utils::Color};
use lavalink_rs::{LavalinkClient, model::{TrackQueue, Node}};

use super::structs::{Lavalink, SerenityContext};

pub async fn exec(cmd: &str, msg: &Message, ctx: &Context, args: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    match cmd {
        "play" | "p" => {
            let channel_id = guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id);

            let connect_to = match channel_id {
                Some(channel) => channel,
                None => {
                    msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                    return Ok(());
                }
            };
        
            let manager = get(&ctx).await.expect("something bad happened").clone();
            let (connection, handler) = manager.join_gateway(guild_id, connect_to).await;
        
            connection.as_ref().lock().await.deafen(true).await?;
        
            match handler {
                Ok(connection_info) => {
                    let data = ctx.data.read().await;
                    let lava_client = data.get::<Lavalink>().unwrap().clone();
                    lava_client.create_session_with_songbird(&connection_info).await?;
                }
                Err(why) => {
                    msg.channel_id.say(ctx, format!("Couldn't join channel: {}", why)).await?;
                },
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
            let query_info = lava_client.auto_search_tracks(&args).await?;
        
            if query_info.tracks.len() == 0 {
                msg.channel_id.say(ctx, "🥟 Can't find song with given name").await?;
                return Ok(());
            }
        
            let track = &query_info.tracks[0];
            let track_title = &track.info.as_ref().unwrap().title;
        
            lava_client.play(guild_id, track.clone()).queue().await?;
        
            if lava_client.nodes().await.get(&guild_id.as_u64()).unwrap().now_playing.is_some() {
                msg.channel_id.say(ctx, format!("🎶 Added to queue: **{}**", track_title)).await?;
                return Ok(());
            }
        
            let node = lava_client.nodes().await;
            let guild_node = node.get(guild_id.as_u64()).unwrap();
            {
                let mut data = guild_node.data.write();
                data.insert::<SerenityContext>((msg.clone(), ctx.http.clone()));
            }
        },

        "pause" => {
            if guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).is_none() {
                msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
        
            lava_client.pause(guild_id).await?;
            msg.channel_id.say(ctx, "⏸ Paused! Type `bb resume` to resume!").await?;
        },

        "resume" => {
            if guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).is_none() {
                msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
        
            lava_client.resume(guild_id).await?;
            msg.channel_id.say(ctx, "⏯ Queue resumed!").await?;
        },

        "skip" | "s" => {
            if guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).is_none() {
                msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                return Ok(());
            }
        
            if !in_voice_channel(ctx, guild_id).await {
                msg.channel_id.say(ctx, "🎶 I'm currently not in any voice channel!").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
        
            if !check_queue(&lava_client, guild_id).await {
                msg.channel_id.say(ctx, "🎞 Queue is empty!").await?;
                return Ok(());
            }
        
            let queue = get_queue(&lava_client, guild_id).await;
            let queue_len = queue.len();
        
            if queue_len == 1 {
                lava_client.destroy(guild_id).await?
            } else if queue_len > 1  {
                lava_client.skip(guild_id).await;
            } else {
                msg.channel_id.say(ctx, "🎞 Queue is empty!").await?;
                return Ok(());
            }
        
            msg.channel_id.say(ctx, "⏩ Skipped!").await?;
        },

        "stop" => {
            if guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).is_none() {
                msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
        
            lava_client.destroy(guild_id).await?;
            {
                let nodes = lava_client.nodes().await;
                nodes.remove(&guild_id.0);
            
                let loops = lava_client.loops().await;
                loops.remove(&guild_id.0);
            }
        
            msg.channel_id.say(ctx, "🎶 I have stopped oki").await?;
        },

        "leave" => {
            let manager = get(&ctx).await.expect("errored").clone();
            let handler = manager.get(guild_id).is_some();

            if handler {
                manager.remove(guild_id).await?;
                msg.channel_id.say(ctx, "🎶 See chu all later!").await?;
                return Ok(());
            }
        
            msg.channel_id.say(ctx, "🎶 I'm currently not in any voice channel!").await?;
        },

        "replay" => {
            if guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).is_none() {
                msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                return Ok(());
            }
        
            if !in_voice_channel(ctx, guild_id).await {
                msg.channel_id.say(ctx, "🎶 I'm currently not in any voice channel!").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
        
            if !check_queue(&lava_client, guild_id).await {
                msg.channel_id.say(ctx, "🎞 Queue is empty!").await?;
                return Ok(());
            }
        
            let node = get_node(&lava_client, guild_id).await;
            let to_replay = &node.now_playing.as_ref().unwrap().track;
        
            lava_client.play(guild_id, to_replay.clone()).replace(true).start().await?;
        
            msg.react(ctx, ReactionType::Unicode("👍".into())).await?;
        },

        "queue" | "q" => {
            if !in_voice_channel(ctx, guild_id).await {
                msg.channel_id.say(ctx, "🎶 I'm currently not in any voice channel!").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
        
            if !check_queue(&lava_client, guild_id).await {
                msg.channel_id.say(ctx, "🎞 Queue is empty!").await?;
                return Ok(());
            }
        
            let queue = get_queue(&lava_client, guild_id).await;
        
            let mut queue_embed = "".into();
            let mut position = 1;
        
            for track in queue.iter() {
                queue_embed = format!("{}\n**{}.** {}", queue_embed, position, track.track.info.as_ref().unwrap().title);
                position += 1
            }
        
            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(format!("🎼 {}'s Current Queue", &guild.name));
                    e.description("🔸 Total length: (todo)");
                    e.color(Color::from_rgb(255, 184, 184));
                    e.field("Now playing:", queue_embed, false);
                    e
                });
                m
            }).await?;
        },

        "songinfo" => {
            if guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).is_none() {
                msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                return Ok(());
            }
        
            if !in_voice_channel(ctx, guild_id).await {
                msg.channel_id.say(ctx, "🎶 I'm currently not in any voice channel!").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();

            if !check_queue(&lava_client, guild_id).await {
                msg.channel_id.say(ctx, "🎞 Queue is empty!").await?;
                return Ok(());
            }
        
            let node = get_node(&lava_client, guild_id).await;
        
            let current = match &node.now_playing {
                Some(track) => &track.track,
                None => {
                    msg.reply(ctx, "🎵 No playing song!").await?;
                    return Ok(());
                }
            };
        
            let info = current.info.as_ref().unwrap().clone();
        
            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(format!("🎻 {}", info.title));
                    e.description(info.author);
                    e.color(Color::from_rgb(255, 184, 184));
                    e.field("Source:", info.uri, false);
                    e
                });
                m
            }).await?;
        },

        "volume" | "vol" => {
            if guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id).is_none() {
                msg.reply(ctx, "🎶 Join a voice channel first pls").await?;
                return Ok(());
            }
        
            if !in_voice_channel(ctx, guild_id).await {
                msg.channel_id.say(ctx, "🎶 I'm currently not in any voice channel!").await?;
                return Ok(());
            }
        
            let lava_client = ctx.data.read().await.get::<Lavalink>().unwrap().clone();
        
            lava_client.volume(guild_id, args.parse::<u16>().unwrap()).await?;
            msg.channel_id.say(ctx, format!("Volume has been set to {}", args)).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn in_voice_channel(ctx: &Context, guild_id: GuildId) -> bool {
    let manager = get(&ctx).await.expect("errored").clone();
    manager.get(guild_id).is_some()
}

async fn get_node(lava_client: &LavalinkClient, guild_id: GuildId) -> Node {
    let nodes = &lava_client.nodes().await;
    let node = &nodes.get(&guild_id.as_u64()).unwrap();

    node.value().clone()
}

async fn get_queue(lava_client: &LavalinkClient, guild_id: GuildId) -> Vec<TrackQueue> {
    let node = get_node(lava_client, guild_id).await;
    let queue = node.queue;

    queue.to_vec()
}

async fn check_queue(lava_client: &LavalinkClient, guild_id: GuildId) -> bool {
    let nodes = &lava_client.nodes().await;
    let node = &nodes.get(&guild_id.as_u64()).is_some();

    node.clone()
}