/**
Misc commands I'm not sure where else to put.
**/
use crate::bot_types::{_Context as Context, Error};
use crate::bot_utils::get_bombs_exploded;
use serenity::all::UserId;

/// Show How many times a user has blown up.
#[poise::command(prefix_command, aliases("blown", "exploded"))]
pub async fn bomb_count(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.channel_id().message(&ctx.http(), ctx.id()).await?;
    let search_user_id: UserId = if msg.referenced_message.is_none() {
        msg.author.id
    } else {
        msg.referenced_message.clone().unwrap().author.id
    };

    let bomb_data = get_bombs_exploded(search_user_id.to_string().as_str()).await;

    let response = format!(
        "{} has stepped on {} bombs.",
        bomb_data.user_name, bomb_data.bombs_exploded
    );
    ctx.reply(response).await?;
    Ok(())
}
