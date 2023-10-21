use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::{Display, Formatter, Result};
use crate::bot_utils::connect_to_database;

/**
Returns the current users score.
**/

struct UserInfo {
    user_name: String,
    score: i64,
}

impl Display for UserInfo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} has {} points", self.user_name, self.score)
    }
}

#[command]
pub async fn score(ctx: &Context, msg: &Message) -> CommandResult {

    let clone_msg = msg;

    let search_user = if msg.referenced_message.is_none() {
        clone_msg.author.id
    } else {
        clone_msg.referenced_message.clone().unwrap().author.id

    };


    let return_user = get_score(search_user.to_string().as_str()).await;

    if let Err(why) = msg.reply(&ctx.http, return_user.to_string()).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

async fn get_score(user: &str) -> UserInfo {
    let database = connect_to_database().await;
    let user = sqlx::query!(
        "SELECT user_name, score FROM user WHERE user_id = ?",
        user,
    )
        .fetch_one(&database)
        .await
        .unwrap();

    return UserInfo {user_name: user.user_name, score: user.score.unwrap()};
}