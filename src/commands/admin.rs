use crate::bot_types::{_Context as Context, Error};
use crate::bot_utils::set_bombs_exploded;
use poise::serenity_prelude as serenity;

/// Just a test command. Does nothing.
#[poise::command(prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    let embed = serenity::CreateEmbed::default().title(response);
    let reply = { poise::CreateReply::default().embed(embed) };

    ctx.send(reply).await?;
    Ok(())
}

/// Sets bomb explode in the datbase.
#[poise::command(
    prefix_command,
    required_permissions = "ADMINISTRATOR",
    aliases("setBomb", "setBombs")
)]
pub async fn set_bombs_for_user(
    ctx: Context<'_>,
    #[description = "The amount to set"] amount: i16,
) -> Result<(), Error> {
    let msg = ctx.channel_id().message(&ctx.http(), ctx.id()).await?;
    if msg.referenced_message.is_none() {
        ctx.reply("No referenced message found").await?;
    }
    let user = msg.referenced_message.unwrap().author;
    set_bombs_exploded(user.clone().id.to_string().as_str(), amount).await;
    let response = format!("Set Bombs for {} to {}", user.name, amount);
    ctx.reply(response).await?;
    Ok(())
}

/// Returns the version of the bot.
#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR")]
pub async fn version(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("The bot is running version {}", env!("CARGO_PKG_VERSION"));
    ctx.reply(response).await?;
    Ok(())
}
