use crate::{Data, Error, _Context as Context};
use poise::serenity_prelude as serenity;

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>,
                  #[description = "Selected User"] user: Option<serenity::User>,
    )-> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
