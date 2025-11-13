mod commands;
mod bot_utils;
mod emoji;
mod bot_types;
// Commands;
use crate::commands::smash::*;
use crate::commands::judge::*;
use crate::commands::score::*;
use crate::commands::ping::*;
use crate::commands::trade::*;
use crate::commands::shop::*;

use crate::bot_types::{Data, Error};

use std::collections::{HashSet};
use serenity::http::*;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::id::{ChannelId, GuildId, MessageId};
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::hook;
use poise::serenity_prelude as serenity_prelude;
use rand::Rng;
use serenity::all::Member;
use serenity::model::Timestamp;
use crate::bot_utils::{get_count, is_bot, reset_count};
use crate::emoji::get_emoji;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache Ready - Environment: {}", bot_utils::get_env());
    }

    /**
    Add new users to the database.
    */
    async fn message(&self, _ctx: Context, msg: Message) {
        bot_utils::create_in_db(&msg.author.id.to_string(), &msg.author.name).await;

        if is_bot(msg.author.id.to_string()) {
            return;
        }

        let mut _rng = rand::rng().random_range(0..100);
        let current_number_of_bombs = get_count("mine").await;
        if _rng <= current_number_of_bombs {
            let mut member = get_member(_ctx.clone(), msg.clone()).await;
            let time_out_time = get_time_out_time();
            member.disable_communication_until_datetime(&_ctx.http.clone(), time_out_time).await.unwrap();
            reset_count("mine").await;
            msg.reply(&_ctx.http, format!("{} You're our lucky loser! See you in 10 minutes. :3", get_emoji("winner"))).await.unwrap();
        }
    }

    async fn reaction_add(&self, _ctx: Context, _add_reaction: Reaction) {
        let reaction = _add_reaction.emoji;
        let message= get_message_from_id(_add_reaction.channel_id, _add_reaction.message_id).await.unwrap().author;
        let score = get_points_from_emoji(reaction);

        if _add_reaction.user_id.unwrap().to_string() == message.id.to_string() {
            return;
        }

        if score == 2 {
            bot_utils::plus_two(&_add_reaction.user_id.unwrap().to_string(), &message.id.to_string(), false).await;
        }

        if score == -2 {
            bot_utils::minus_two(&_add_reaction.user_id.unwrap().to_string(), &message.id.to_string(), false).await;
        }

        bot_utils::score_update(&message.id.to_string(), score).await;

    }

    async fn reaction_remove(&self, _ctx: Context, _removed_reaction: Reaction) {
        let reaction = _removed_reaction.emoji;
        let message= get_message_from_id(_removed_reaction.channel_id, _removed_reaction.message_id).await.unwrap().author;
        let score = get_points_from_emoji(reaction);

        if _removed_reaction.user_id.unwrap().to_string() == message.id.to_string() {
            return;
        }

        if score == 2 {
            bot_utils::plus_two(&_removed_reaction.user_id.unwrap().to_string(), &message.id.to_string(), true).await;
        }

        if score == -2 {
            bot_utils::minus_two(&_removed_reaction.user_id.unwrap().to_string(), &message.id.to_string(), true).await;
        }

        bot_utils::score_update(&message.id.to_string(), score * -1).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected! Environment: {}", ready.user.name, bot_utils::get_env());
    }

}


/**
Returns a time 10 minutes from now.
**/
fn get_time_out_time() -> Timestamp {
    let current_time: i64 = Timestamp::now().unix_timestamp();
    let time_out = 600;
    Timestamp::from_unix_timestamp(current_time + time_out as i64).unwrap()
}

/**
Returns the Member of the message sent.
**/
async fn get_member(_ctx: Context, msg: Message) -> Member {
    let guild_id = msg.guild_id.unwrap();
    let member = guild_id.member(&_ctx.http, msg.author.id).await.unwrap();
    member
}

fn get_points_from_emoji(reaction: ReactionType) -> i16 {
    let mut score:i16 = 0;
    if reaction == emoji::get_emoji("plus_two") || reaction == emoji::get_emoji("manny") {
        score = 2;
    }
    if reaction == emoji::get_emoji("minus_two") || reaction == emoji::get_emoji("doot") {
        score = -2;
    }
    return score;
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[tokio::main]
async fn main() {
    let token = bot_utils::get_secret();
    let http = Http::new(&token);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    let (owners, bot_id) = match http.get_current_application_info().await
    {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.unwrap().id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), judge(), score(), top(), leader(), smash(), trade(), wallet(), shop(), item_count()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework|{
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity_prelude::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await;
    client.unwrap().start().await.unwrap();



}
async fn get_message_from_id(channel_id:ChannelId, message_id: MessageId) -> serenity::Result<Message> {
    let token = bot_utils::get_secret();
    let http = Http::new(&token);
    let message = channel_id.message(&http, message_id);
    return message.await;
}

