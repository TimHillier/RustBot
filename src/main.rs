mod bot_types;
mod bot_utils;
mod commands;
mod emoji;
mod runescape_utils;

// Commands;
use crate::commands::admin::*;
use crate::commands::help::*;
use crate::commands::image::*;
use crate::commands::judge::*;
use crate::commands::misc::*;
use crate::commands::runescape::*;
use crate::commands::score::*;
use crate::commands::shop::*;
use crate::commands::smash::*;
use crate::commands::trade::*;

use crate::bot_types::{Data, Error};

use crate::bot_utils::{
    get_count, get_current_bot_id, increase_bombs_exploded, is_bot, reset_count, score_update,
};
use crate::emoji::get_emoji;
use poise::serenity_prelude;
use rand::Rng;
use serenity::all::Member;
use serenity::async_trait;
use serenity::framework::standard::macros::hook;
use serenity::http::*;
use serenity::model::Timestamp;
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId, MessageId};
use serenity::prelude::*;

struct Handler;
const MAX_BOMB_RANGE: i64 = 300;
const BOMB_POINTS_LOST: i16 = 20;

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

        let mut _rng = rand::rng().random_range(0..MAX_BOMB_RANGE);
        let current_number_of_bombs = get_count("mine").await;
        if _rng <= current_number_of_bombs {
            let current_bot_id = get_current_bot_id().await.to_string();
            let mut member = get_member(_ctx.clone(), msg.clone()).await;
            let time_out_time = get_time_out_time();
            member
                .disable_communication_until_datetime(&_ctx.http.clone(), time_out_time)
                .await
                .unwrap();
            reset_count("mine").await;
            increase_bombs_exploded(&msg.author.id.to_string()).await;
            do_transaction(
                &msg.author.id.to_string(),
                &current_bot_id,
                BOMB_POINTS_LOST,
            )
            .await;

            msg.reply(
                &_ctx.http,
                format!(
                    "{} oh nyo, >w< wooks wike somebwody got bwown up and wost {} pwoints. ",
                    get_emoji("winner"),
                    BOMB_POINTS_LOST,
                ),
            )
            .await
            .unwrap();
        }

        // Rotate the image sometimes.
        if !msg.attachments.is_empty() {
            let mut _rng = rand::rng().random_range(0..=100);
            let lucky_numbers = [7];
            if lucky_numbers.contains(&_rng) {
                rotate_image_directly(&_ctx.http, msg.channel_id, &msg)
                    .await
                    .expect("Error rotating image");
            }
        }
    }

    async fn reaction_add(&self, _ctx: Context, _add_reaction: Reaction) {
        let reaction = _add_reaction.emoji;
        let message = get_message_from_id(_add_reaction.channel_id, _add_reaction.message_id)
            .await
            .unwrap()
            .author;
        let score = get_points_from_emoji(&reaction);

        if _add_reaction.user_id.unwrap().to_string() == message.id.to_string() {
            // Don't let the message owner add a reaction to themselves.
            return;
        }

        if score == 2 {
            bot_utils::plus_two(
                &_add_reaction.user_id.unwrap().to_string(),
                &message.id.to_string(),
                false,
            )
            .await;
        }

        if score == -2 {
            bot_utils::minus_two(
                &_add_reaction.user_id.unwrap().to_string(),
                &message.id.to_string(),
                false,
            )
            .await;
        }

        score_update(&message.id.to_string(), score).await;
    }

    async fn reaction_remove(&self, _ctx: Context, _removed_reaction: Reaction) {
        let reaction = _removed_reaction.emoji;
        let message =
            get_message_from_id(_removed_reaction.channel_id, _removed_reaction.message_id)
                .await
                .unwrap()
                .author;
        let score = get_points_from_emoji(&reaction);

        if _removed_reaction.user_id.unwrap().to_string() == message.id.to_string() {
            // Don't let the message owner remove a reaction from themselves.
            return;
        }

        if score == 2 {
            bot_utils::plus_two(
                &_removed_reaction.user_id.unwrap().to_string(),
                &message.id.to_string(),
                true,
            )
            .await;
        }

        if score == -2 {
            bot_utils::minus_two(
                &_removed_reaction.user_id.unwrap().to_string(),
                &message.id.to_string(),
                true,
            )
            .await;
        }

        bot_utils::score_update(&message.id.to_string(), -score).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{} is connected! Environment: {}",
            ready.user.name,
            bot_utils::get_env()
        );
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
    guild_id.member(&_ctx.http, msg.author.id).await.unwrap()
}

fn get_points_from_emoji(reaction: &ReactionType) -> i16 {
    emoji::points_from_reaction(reaction)
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[tokio::main]
async fn main() {
    let token = bot_utils::get_secret();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    // TODO make commands combine vectors from all the command files.
    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                version(),
                judge(),
                score(),
                top(),
                leader(),
                smash(),
                trade(),
                wallet(),
                rotate(),
                check_folder(),
                shop(),
                count(),
                help(),
                grand_exchange(),
                grand_exchange_history(),
                ge_set_alias(),
                lookup_alias(),
                set_bombs_for_user(),
                bomb_count(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
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
async fn get_message_from_id(
    channel_id: ChannelId,
    message_id: MessageId,
) -> serenity::Result<Message> {
    let token = bot_utils::get_secret();
    let http = Http::new(&token);
    let message = channel_id.message(&http, message_id);
    return message.await;
}
