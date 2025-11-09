/**
Returns smash or pass.
 - Requires image?
 - Should return in reply.
 **/
use crate::bot_utils;
use crate::bot_types::{Error, _Context as Context};

#[poise::command(prefix_command)]
pub async fn smash(ctx: Context<'_>) -> Result<(), Error>{
    let mut reply = if bot_utils::get_random_bool(0.5) {"Smash"} else {"Pass"};

    if bot_utils::get_random_bool(0.2) {
        reply = if bot_utils::get_random_bool(0.5) {"Easy smash"} else {"Hard pass"};
    }

    if let Err(why) = ctx.reply(reply).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}