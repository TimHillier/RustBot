/**
Returns smash or pass.
 - Requires image?
 - Should return in reply.
 **/
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::bot_utils;

#[command]
pub async fn smash(ctx: &Context, msg:&Message) -> CommandResult {
    let mut reply = if bot_utils::get_random_bool(0.5) {"Smash"} else {"Pass"};

    if bot_utils::get_random_bool(0.2) {
        reply = if bot_utils::get_random_bool(0.5) {"Easy smash"} else {"Hard pass"};
    }

    if let Err(why) = msg.reply(&ctx.http, reply).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}