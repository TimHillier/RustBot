use crate::{bot_utils, emoji};
use crate::bot_utils::{connect_to_database};
use rand::seq::IndexedRandom;
use crate::bot_types::{ Error, _Context as Context};
use poise::{ serenity_prelude as serenity};
use serenity::model::channel::ReactionType;

#[poise::command(prefix_command)]
pub async fn judge(ctx: Context<'_>) -> Result<(), Error> {

    let msg = ctx.channel_id().message(&ctx.http(), ctx.id()).await?;
    // Add Emojis to Judge Command.
    let mut emojis: Vec<ReactionType> = vec![];
    let current_env = bot_utils::get_env();
    if String::from("live").eq(&current_env) {
        emojis.push(emoji::get_emoji("plus_two"));
        emojis.push(emoji::get_emoji("minus_two"));
    } else {
        emojis.push(emoji::get_emoji("manny"));
        emojis.push(emoji::get_emoji("doot"));
    }
    let reaction = emojis.choose(&mut rand::rng()).unwrap().clone();

    if msg.referenced_message.is_none() {
        if let Err(why) = ctx.reply("Command can only be used as a reply.").await {
            println!("Error sending message: {:?}", why);
        }
        Ok(())
    } else if has_been_judged(&msg.referenced_message.clone().unwrap().id.to_string()).await {
            if let Err(why) = ctx.reply("Post has already been judged.").await {
                println!("Error sending message: {:?}", why);
            }
            Ok(())
        } else {
        if let Err(why) = msg.referenced_message.clone().unwrap().react(&ctx.http(), reaction.clone()).await {
            println!("Error sending message: {:?}", why);
        }

        let message_id = &msg.referenced_message.clone().unwrap().id.to_string();
        let message_owner = &msg.referenced_message.clone().unwrap().author.name;
        let command_caller = &msg.author.name;
        let result = &reaction.to_string();

        insert_into_judged(message_id, message_owner, command_caller, result).await;

        Ok(())
    }

}

async fn insert_into_judged(message_id:&str, message_owner:&str, command_caller: &str, result: &str) {
    let database = connect_to_database().await;
    sqlx::query!(
            "INSERT INTO judgedPosts (message_id, message_owner, command_caller, result) VALUES (?, ?, ?, ?)",
            message_id,
            message_owner,
            command_caller,
            result
        )
        .execute(&database)
        .await
        .unwrap();
}

async fn has_been_judged(message_id:&str) -> bool {
    let database = connect_to_database().await;
    let is_judged = sqlx::query!(
            "SELECT message_id FROM judgedPosts WHERE message_id = ?",
            message_id,
        )
        .fetch_all(&database)
        .await
        .unwrap();

   if is_judged.len() >= 1 {
       return true;
   }
    return false;
}
