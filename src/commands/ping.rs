use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;

#[command]
pub async fn ping(ctx: &Context, msg:&Message) -> CommandResult {
    // if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
    //     println!("Error sending message: {:?}", why);
    // }

    let doot_reaction = ReactionType::Custom {
        animated: false,
        id: EmojiId(929985012554682469),
        name: Some("doot".to_string()),
    };

    // how to add reactions to message?
    if let Err(why) = msg.react(&ctx.http, doot_reaction).await {
            println!("Error sending message: {:?}", why);
    }
    Ok(())
}