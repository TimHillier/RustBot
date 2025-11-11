use crate::bot_types::{_Context as Context, Error};
use crate::bot_utils;
use crate::emoji;
use crate::emoji::get_emoji;
use poise::serenity_prelude as serenity;
use bot_utils::get_score;

/**
Trades +2's from the caller, to the reviver.
 **/
#[poise::command(prefix_command)]
pub async fn trade(
    ctx: Context<'_>,
    #[description = "User to trade with"] user: Option<serenity::User>,
    #[description = "Amount of +2's to trade"] amount: String,
) -> Result<(), Error> {
    let from_user_id = ctx.author().id.to_string();
    let receiving_user_id = user.clone().unwrap().id.to_string();
    let number_amount = amount.parse::<i16>();

    // no funny business
    if number_amount.clone()? < 0 {
        let msg = ctx.channel_id().message(&ctx.http(), ctx.id()).await?;
        ctx.reply("Nope.").await.expect("Error with 0 handling");
        msg.react(&ctx.http(), get_emoji("minus_two"))
            .await
            .expect("Error with 0 handling -2");
        return Ok(());
    }

    // Check that the user has enough +2's
    let from_user_p2_count = bot_utils::get_plus_two_received(from_user_id.clone())
        .await
        .unwrap();
    if from_user_p2_count < number_amount.clone()?.into() {
        ctx.reply(format!(
            "{} only has {} {}. Not enough to continue this transaction.",
            ctx.author().name,
            from_user_p2_count,
            emoji::get_emoji("plus_two")
        ))
        .await
        .expect("Could not get Plus Two Count.");
        return Ok(());
    }

    let message_id = ctx.id().to_string();
    let from_user = &ctx.author().name;
    let receiving_user = user.unwrap().name;
    do_transaction(
        &from_user_id,
        &receiving_user_id,
        number_amount.clone().unwrap(),
    )
    .await;
    bot_utils::add_trade_log(message_id, from_user, receiving_user.clone(), amount.clone()).await;
    ctx.reply(format!(
        "{} has traded {} {} {}. The updated scores are {}: {} and {}: {}",
        from_user.clone(),
        receiving_user.clone(),
        amount.clone(),
        get_emoji("plus_two"),
        from_user,
        get_score(from_user_id.as_str()).await.to_string(),
        receiving_user,
        get_score(receiving_user_id.as_str()).await.to_string()
    )).await.expect("Error Updating Score");
    Ok(())
}

async fn do_transaction(giver_id: &str, receiver_id: &str, amount: i16) {
    bot_utils::take_plus_two(giver_id, amount).await;
    bot_utils::give_plus_two(receiver_id, amount).await;
    bot_utils::score_update(giver_id, amount * -2).await;
    bot_utils::score_update(receiver_id, amount * 2).await;
}
