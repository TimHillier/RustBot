use crate::bot_types::{Error, _Context as Context };
use poise::serenity_prelude as serenity;
use serenity::all::Timestamp;

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>,
                  #[description = "Selected User"] user: Option<serenity::User>,
    )-> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    let embed = serenity::CreateEmbed::default().title(response);
    let reply = {
        poise::CreateReply::default()
            .embed(embed)
    };

    ctx.send(reply).await?;
    Ok(())
}

// #[poise::command(prefix_command)]
pub async fn time_out_user(ctx: Context<'_>) -> Result<(), Error> {
    println!("Time out user");
    let now = ctx.created_at().unix_timestamp();
    let total_seconds = 10;
    let then = Timestamp::from_unix_timestamp(now + total_seconds as i64)?;

    let mut member = ctx
        .author_member()
        .await
        .ok_or("Error Getting Member")?
        .into_owned();

    member
        .disable_communication_until_datetime(&ctx, then)
        .await?;

    println!("{} has been Timed Out! until {}", ctx.author().name, then);
    ctx.reply(format!("Timed Out!")).await?;

    Ok(())
}
