/**
Commands for displaying scores.
**/

use std::fmt::{Display, Formatter, Result as fmtResult};
use crate::bot_types::{ Error, _Context as Context };
use crate::bot_utils::{get_user_info_score, get_top_scores};

/**
Struct used for displaying Information
**/
pub struct UserInfo {
    pub(crate) user_name: String,
    pub(crate) score: i64,
}

impl Display for UserInfo {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, " - {} > {}\n", self.user_name, self.score)
    }
}

pub(crate) struct UserInfoVec(pub Vec<UserInfo>);
impl Display for UserInfoVec {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
       let mut comma_seperated = String::new();

        for val in &self.0[0..self.0.len()-1] {
            comma_seperated.push_str(val.user_name.to_string().as_str());
            comma_seperated.push_str(val.score.to_string().as_str());
            comma_seperated.push_str(", ");
        }
        comma_seperated.push_str(&self.0[self.0.len() -1].to_string());
        write!(f, "{}", comma_seperated)
    }
}
/**
Returns the Top Scoring User.
**/
#[poise::command(prefix_command)]
pub async fn top(ctx: Context<'_>) -> Result<(), Error>{
    let top_scores = get_top_scores(1).await;

    let mut reply_string: String = String::new();
    for (i, value) in top_scores.0.iter().enumerate() {
        reply_string.push_str((i+1).to_string().as_str());
        reply_string.push_str(value.to_string().as_str());
    }

    if let Err(why) = ctx.reply( reply_string).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

/**
Returns the top 10 scoring users.
**/
#[poise::command(prefix_command, aliases("board", "leaderboard", "lb"))]
pub async fn leader(ctx: Context<'_>) -> Result<(), Error>{
    let top_scores = get_top_scores(10).await;


    let mut reply_string: String = String::new();
    for (i, value) in top_scores.0.iter().enumerate() {
        reply_string.push_str((i+1).to_string().as_str());
        reply_string.push_str(value.to_string().as_str());
    }

    if let Err(why) = ctx.reply(reply_string).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

/**
Returns the score of a specific user.
**/
#[poise::command(prefix_command)]
pub async fn score(ctx: Context<'_>) -> Result<(), Error> {

    let msg = ctx.channel_id().message(&ctx.http(), ctx.id()).await?;

    let search_user = if msg.referenced_message.is_none() {
        msg.author.id
    } else {
        msg.referenced_message.clone().unwrap().author.id

    };


    let return_user = get_user_info_score(search_user.to_string().as_str()).await;

    if let Err(why) = ctx.reply( return_user.to_string()).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
