mod commands;
mod bot_utils;
mod emoji;

// Commands;
use crate::commands::ping::*;
use crate::commands::smash::*;
use crate::commands::judge::*;
use crate::commands::score::*;
use crate::commands::top::*;

use std::collections::{HashSet};
use serenity::http::*;
use serenity::framework::StandardFramework;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::{group, hook};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        bot_utils::score_insert(msg.author.id.0.to_string().as_str(), msg.author.name.as_str()).await;
    }

    async fn reaction_add(&self, _ctx: Context, _add_reaction: Reaction) {
        let reaction = _add_reaction.emoji;
        let message= get_message_from_id(_add_reaction.channel_id, _add_reaction.message_id).await.unwrap().author;
        let score = get_points_from_emoji(reaction);

        if &_add_reaction.user_id.unwrap().0.to_string().as_str() == &message.id.0.to_string().as_str() {
            return;
        }

        if score == 2 {
            bot_utils::plus_two(_add_reaction.user_id.unwrap().0.to_string().as_str(), &message.id.0.to_string().as_str(), false).await;
        }

        if score == -2 {
            bot_utils::minus_two(_add_reaction.user_id.unwrap().0.to_string().as_str(), &message.id.0.to_string().as_str(), false).await;
        }

        bot_utils::score_update(message.id.0.to_string().as_str(), score).await;

    }

    async fn reaction_remove(&self, _ctx: Context, _removed_reaction: Reaction) {
        let reaction = _removed_reaction.emoji;
        let message= get_message_from_id(_removed_reaction.channel_id, _removed_reaction.message_id).await.unwrap().author;
        let score = get_points_from_emoji(reaction);

        if &_removed_reaction.user_id.unwrap().0.to_string().as_str() == &message.id.0.to_string().as_str() {
            return;
        }

        if score == 2 {
            bot_utils::plus_two(_removed_reaction.user_id.unwrap().0.to_string().as_str(), &message.id.0.to_string().as_str(), true).await;
        }

        if score == -2 {
            bot_utils::minus_two(_removed_reaction.user_id.unwrap().0.to_string().as_str(), &message.id.0.to_string().as_str(), true).await;
        }

        bot_utils::score_update(message.id.0.to_string().as_str(), score * -1).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

}

fn get_points_from_emoji(reaction: ReactionType) -> i8 {
    let mut score:i8 = 0;
    if reaction == emoji::get_emoji("plus_two") || reaction == emoji::get_emoji("manny") {
        score = 2;
    }
    if reaction == emoji::get_emoji("minus_two") || reaction == emoji::get_emoji("doot") {
        score = -2;
    }
    return score;
}

struct Bot {
    database: sqlx::SqlitePool,
}

#[group]
#[commands(smash, judge, score, top, leader)]
struct General;

#[group]
#[owners_only]
#[only_in(guilds)]
struct Owner;

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[tokio::main]
async fn main() {
    let token = bot_utils::get_secret();
    let http = Http::new(&token);
    let database = bot_utils::connect_to_database().await;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;

    let (owners, bot_id) = match http.get_current_application_info().await
    {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("!")
            .delimiters(vec![",",","])
            .owners(owners))
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP);

    let mut client =
        Client::builder(&token, intents)
            .framework(framework)
            .event_handler(Handler)
            .await.expect("Err creating client");


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
async fn get_message_from_id(channel_id:ChannelId, message_id: MessageId) -> serenity::Result<Message> {
    let token = bot_utils::get_secret();
    let http = Http::new(&token);
    let message = channel_id.message(&http, message_id);
    return message.await;
}

