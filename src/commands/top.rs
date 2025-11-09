/**
Returns the Top 3 scores.
**/

use std::fmt::{Display, Formatter, Result as fmtResult};
use crate::bot_types::{ Error, _Context as Context};
use crate::bot_utils::connect_to_database;

#[derive(Debug)]
struct UserInfo {
    user_name: String,
    score: i64,
}

impl Display for UserInfo {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, " - {} > {}\n", self.user_name, self.score)
    }
}

#[derive(Debug)]
struct UserInfoVec(Vec<UserInfo>);
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

#[poise::command(prefix_command)]
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
Returns users : scores with the top N results.
**/
async fn get_top_scores(limit: i8) -> UserInfoVec {
    let database = connect_to_database().await;
    let top = sqlx::query!(
            "SELECT user_name, score FROM user ORDER BY score DESC LIMIT ?",
        limit
    )
        .fetch_all(&database)
        .await
        .unwrap();


    let mut user_vector = UserInfoVec(vec![]);
    for value in top.iter() {
        let temp_user = UserInfo {user_name: value.user_name.to_string(), score: value.score.unwrap()};
        user_vector.0.push(temp_user)
    }

    return user_vector;
}