/**
Commands for voice channels.
**/
use serenity::model::prelude::*;
use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{Args, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::bot_utils::{check_msg};
#[command]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg:&Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "You're not in a voice channel").await);
            return Ok(());
        }
    };

        let manager = songbird::get(ctx).await.expect("Songbird Voice client placed in at initialisation").clone();
        let _handler = manager.join(guild_id, connect_to).await;
        Ok(())

}

#[command]
#[only_in(guilds)]
pub async fn leave(ctx: &Context, msg:&Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.expect("Songbird Voice Client placed in at initalisation").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed {:?}", e)).await);
        }
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn stop(ctx: &Context, msg:&Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice Client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        handler.queue().pause().expect("Error Stopping Song");
    } else {
        check_msg(msg.reply(&ctx.http, "Not in voice channel.").await);
    }

    Ok(())
}


#[command]
#[only_in(guilds)]
pub async fn queue(ctx: &Context, msg:&Message, mut args: Args) -> CommandResult {
    play_or_queue(ctx, msg, args).await.expect("Error With Track");
    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg:&Message, mut args: Args) -> CommandResult {
    play_or_queue(ctx, msg, args).await.expect("Error With Track");
    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn skip(ctx: &Context, msg:&Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        check_msg(msg.reply(&ctx.http, "Song skipped").await);
    } else {
        check_msg(msg.reply(&ctx.http, "Not in Voice Channel").await);
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn resume(ctx: &Context, msg:&Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.resume();
    } else {
        check_msg(msg.reply(&ctx.http, "Not in Voice Channel").await);
    }
    Ok(())
}

pub async fn play_or_queue(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.reply(&ctx.http, "Must provide a Url.").await);

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        check_msg(msg.reply(&ctx.http, "Must provide a valid Url. (http://...)").await);
        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.expect("Songbird Voice Client placed in at initalisation").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Play Command Error: {:?}", why);
                check_msg(msg.reply(&ctx.http, "Error").await);
                return Ok(())
            },
        };


        if !handler.queue().is_empty() {
            check_msg(msg.reply(&ctx.http, format!("Added song to queue: position {}", handler.queue().len())).await,);
        }

        handler.enqueue_source(source.into());

    } else {
        check_msg(msg.reply(&ctx.http, "Not in voice channel.").await);
    }

    Ok(())
}

