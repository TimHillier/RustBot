use crate::UserInfo;
use crate::commands::shop::ItemInfo;
use rand::Rng;
use serde::Deserialize;
use serenity::model::channel::{Attachment, Message};
use sqlx::{Pool, Sqlite};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
struct SecretsToml {
    #[allow(dead_code)]
    discord_token: String,
    environment: String,
    live_bot_user_id: String,
    testing_bot_user_id: String,
}

pub fn get_toml() -> String {
    fs::read_to_string("data/Secrets.toml").expect("Failed to read TOML")
}
pub fn get_secret() -> String {
    let toml_str = get_toml();
    let secrets_toml: SecretsToml = toml::from_str(&toml_str).expect("Failed to decode toml");
    secrets_toml.discord_token
}

pub fn get_env() -> String {
    let toml_str = get_toml();
    let secrets_toml: SecretsToml = toml::from_str(&toml_str).expect("Failed to decode toml");
    let environment = secrets_toml.environment;
    if environment.is_empty() {
        return String::from("testing");
    }
    environment
}

pub fn is_bot(id: String) -> bool {
    let toml_str = get_toml();
    let secrets_toml: SecretsToml = toml::from_str(&toml_str).expect("Failed to decode toml");
    if (id != secrets_toml.testing_bot_user_id) && (id != secrets_toml.live_bot_user_id) {
        return false;
    }
    true
}

pub fn get_random_bool(prob: f64) -> bool {
    let mut rng = rand::rng();
    rng.random_bool(prob)
}

// A connection to the database.
pub async fn connect_to_database() -> Pool<Sqlite> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("data/rustbot.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't Connect to database.")
}

pub async fn score_update(user_id: &str, points: i16) {
    let database = connect_to_database().await;

    sqlx::query!(
        "UPDATE user SET score = score + ? WHERE user_id = ?",
        points,
        user_id
    )
    .execute(&database)
    .await
    .expect("Couldn't increase users score.");
}

pub async fn gave_plus_two(user_id: &str, removed: bool) {
    let mut increase = 1;
    if removed {
        increase = -1;
    }
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET plus_two_given = plus_two_given + ? WHERE user_id = ?",
        increase,
        user_id
    )
    .execute(&database)
    .await
    .expect("Couldn't give plus two");
}
pub async fn gave_minus_two(user_id: &str, removed: bool) {
    let mut increase = 1;
    if removed {
        increase = -1;
    }
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET minus_two_given = minus_two_given + ? WHERE user_id = ?",
        increase,
        user_id
    )
    .execute(&database)
    .await
    .expect("Couldn't give minus two");
}
pub async fn received_plus_two(user_id: &str, removed: bool) {
    let mut increase = 1;
    if removed {
        increase = -1;
    }
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET plus_two_received = plus_two_received + ? WHERE user_id = ?",
        increase,
        user_id
    )
    .execute(&database)
    .await
    .unwrap();
}
pub async fn received_minus_two(user_id: &str, removed: bool) {
    let mut increase = 1;
    if removed {
        increase = -1;
    }
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET minus_two_received = minus_two_received + ? WHERE user_id = ?",
        increase,
        user_id
    )
    .execute(&database)
    .await
    .unwrap();
}

pub async fn plus_two(giver_id: &str, received_id: &str, removed: bool) {
    gave_plus_two(giver_id, removed).await;
    received_plus_two(received_id, removed).await;
}

pub async fn minus_two(giver_id: &str, received_id: &str, removed: bool) {
    gave_minus_two(giver_id, removed).await;
    received_minus_two(received_id, removed).await;
}

/**
Directly give the user_id plus 2's
**/
pub async fn give_plus_two(user_id: &str, amount_given: i16) {
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET plus_two_received = plus_two_received + ? WHERE user_id = ?",
        amount_given,
        user_id
    )
    .execute(&database)
    .await
    .expect("Couldn't give plus two");
}

/**
Get the current amount of plus 2's the user has.
**/
pub async fn get_plus_two_received(user_id: String) -> Option<i64> {
    let database = connect_to_database().await;
    let plus_2_amount = sqlx::query!(
        "SELECT plus_two_received FROM user WHERE user_id = ?",
        user_id
    )
    .fetch_one(&database)
    .await
    .unwrap();

    plus_2_amount.plus_two_received
}

/**
Directly take the user_id plus 2's
**/
pub async fn take_plus_two(user_id: &str, amount_taken: i16) {
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET plus_two_received = plus_two_received - ? WHERE user_id = ?",
        amount_taken,
        user_id
    )
    .execute(&database)
    .await
    .expect("Couldn't take plus two");
}

/**
Get the users score formated for userInfo.
**/
pub async fn get_user_info_score(user: &str) -> UserInfo {
    let database = connect_to_database().await;
    let user = sqlx::query!("SELECT user_name, score FROM user WHERE user_id = ?", user,)
        .fetch_one(&database)
        .await
        .unwrap();

    UserInfo {
        user_name: user.user_name,
        score: user.score.unwrap(),
    }
}

/**
Get the number of times a user has stepped on a bomb.
**/
pub struct BombData {
    pub user_name: String,
    pub bombs_exploded: i64,
}
pub async fn get_bombs_exploded(user_id: &str) -> BombData {
    let database = connect_to_database().await;
    let user = sqlx::query!(
        "SELECT user_name, bombs_exploded FROM user WHERE user_id = ?",
        user_id,
    )
    .fetch_one(&database)
    .await
    .unwrap();

    BombData {
        user_name: user.user_name,
        bombs_exploded: user.bombs_exploded.unwrap(),
    }
}

/**
Set the number of times a user has stepped on a bomb.
**/
pub async fn set_bombs_exploded(user_id: &str, amount: i16) {
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET bombs_exploded = ? WHERE user_id = ?",
        amount,
        user_id,
    )
    .execute(&database)
    .await
    .expect("Couldn't set bomb amount");
}

/**
Increase the number of times a user has stepped on a bomb.
**/
pub async fn increase_bombs_exploded(user_id: &str) {
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET bombs_exploded = bombs_exploded + 1 WHERE user_id = ?",
        user_id,
    )
    .execute(&database)
    .await
    .expect("Couldn't increase bomb amount");
}

/**
Get the users score formated for userInfo.
**/
pub async fn get_score(user: &str) -> i64 {
    let database = connect_to_database().await;
    let result = sqlx::query!("SELECT score FROM user WHERE user_id = ?", user,)
        .fetch_one(&database)
        .await
        .unwrap();

    result.score.unwrap()
}

/**
Returns users : scores with the top N results.
**/
pub(crate) async fn get_top_scores(limit: i8) -> crate::commands::score::UserInfoVec {
    let database = connect_to_database().await;
    let top = sqlx::query!(
        "SELECT user_name, score FROM user ORDER BY score DESC LIMIT ?",
        limit
    )
    .fetch_all(&database)
    .await
    .unwrap();

    let mut user_vector = crate::commands::score::UserInfoVec(vec![]);
    for value in top.iter() {
        let temp_user = UserInfo {
            user_name: value.user_name.to_string(),
            score: value.score.unwrap(),
        };
        user_vector.0.push(temp_user)
    }

    user_vector
}

/**
This creates a new user in the db if they already don't exist.
**/
pub async fn create_in_db(user_id: &str, user_name: &str) {
    let database = connect_to_database().await;
    sqlx::query!(
            "INSERT OR IGNORE INTO user (user_id, user_name, score, plus_two_given, plus_two_received, minus_two_given, minus_two_received) VALUES (?, ?, ?, ?, ?, ?, ?)",
            user_id,
            user_name,
            0,
            0,
            0,
            0,
            0,
        )
        .execute(&database)
        .await
        .unwrap();
}

/**
Add log to the trade logtable.
**/
pub async fn add_trade_log(
    message_id: String,
    from_user: &str,
    receiving_user: String,
    amount: String,
) {
    let database = connect_to_database().await;
    sqlx::query!(
        "INSERT INTO tradeLogs (message_id, from_user, receiving_user, amount ) VALUES (?, ?, ?, ?)",
        message_id,
        from_user,
        receiving_user,
        amount,
    )
        .execute(&database)
        .await
        .unwrap();
}

/**
Get Items from the shop table.
**/
pub async fn get_shop_items() -> crate::commands::shop::ItemInfoVec {
    let database = connect_to_database().await;
    let shop_items = sqlx::query!(
        "SELECT item_name, price, short_name, description FROM shop_items ORDER BY price DESC",
    )
    .fetch_all(&database)
    .await
    .unwrap();

    let mut item_vector = crate::commands::shop::ItemInfoVec(vec![]);
    for item in shop_items.iter() {
        let temp_item = ItemInfo {
            item_name: item.item_name.to_string(),
            short_name: item.short_name.to_string(),
            price: item.price.to_string(),
            description: item.description.to_string(),
        };
        item_vector.0.push(temp_item)
    }
    item_vector
}

/**
Returns the current number of active Bombs.
**/
pub async fn get_count(item: &str) -> i64 {
    let database = connect_to_database().await;
    let number_of_mines = sqlx::query!(
        "SELECT current_amount FROM shop_items WHERE short_name = ?",
        item
    )
    .fetch_one(&database)
    .await
    .unwrap();

    number_of_mines.current_amount
}

/**
Resets the current number of active Bombs back to 1.
**/
pub async fn reset_count(item: &str) {
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE shop_items SET current_amount = 1 WHERE short_name = ?",
        item
    )
    .execute(&database)
    .await
    .unwrap();
}

pub async fn get_current_bot_id() -> String {
    let toml_str = get_toml();
    let secrets_toml: SecretsToml = toml::from_str(&toml_str).expect("Failed to decode toml");
    if get_env() == "live" {
        return secrets_toml.live_bot_user_id;
    }
    secrets_toml.testing_bot_user_id
}

const MESSAGE_ATTACHMENTS_DIR: &str = "data/message_attachments";
const MAX_ATTACHMENT_BYTES: u64 = 50_000_000;

pub async fn add_message_to_db(message: Message) {
    let database = connect_to_database().await;
    let message_id = message.id.to_string();
    let user_id = message.author.id.to_string();
    let timestamp = message.timestamp.to_string();
    let channel_id = message.channel_id.to_string();
    let guild_id = message.guild_id.map(|id| id.to_string());
    let author_name = message.author.name.clone();
    let author_global_name = message.author.global_name.clone();
    let author_is_bot = i64::from(message.author.bot);
    let kind = format!("{:?}", message.kind);
    let referenced_message_id = message
        .message_reference
        .as_ref()
        .and_then(|reference| reference.message_id)
        .map(|id| id.to_string());
    let edited_timestamp = message.edited_timestamp.map(|ts| ts.to_string());
    sqlx::query!(
        "INSERT INTO messages (message_id, user_id, message_content, timestamp, channel_id, guild_id, author_name, author_global_name, is_bot, kind, referenced_message_id, edited_timestamp) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        message_id,
        user_id,
        message.content,
        timestamp,
        channel_id,
        guild_id,
        author_name,
        author_global_name,
        author_is_bot,
        kind,
        referenced_message_id,
        edited_timestamp,
    )
    .execute(&database)
    .await
    .unwrap();

    save_message_attachments(&database, &message_id, &message.attachments).await;
}

async fn save_message_attachments(
    database: &Pool<Sqlite>,
    message_id: &str,
    attachments: &[Attachment],
) {
    for attachment in attachments {
        let attachment_id = attachment.id.to_string();
        let filename = attachment.filename.clone();
        let content_type = attachment.content_type.clone();
        let size = i64::from(attachment.size);
        let url = attachment.url.clone();
        let proxy_url = attachment.proxy_url.clone();
        let width = attachment.width.map(i64::from);
        let height = attachment.height.map(i64::from);
        let local_path = download_message_attachment(message_id, attachment).await;

        sqlx::query!(
            "INSERT INTO message_attachments (attachment_id, message_id, filename, content_type, size, url, proxy_url, width, height, local_path) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            attachment_id,
            message_id,
            filename,
            content_type,
            size,
            url,
            proxy_url,
            width,
            height,
            local_path,
        )
        .execute(database)
        .await
        .expect("Couldn't insert message attachment.");
    }
}

fn attachment_file_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| {
            !ext.is_empty() && ext.len() <= 8 && ext.chars().all(|c| c.is_ascii_alphanumeric())
        })
        .map(|ext| format!(".{ext}"))
        .unwrap_or_default()
}

async fn download_message_attachment(message_id: &str, attachment: &Attachment) -> Option<String> {
    if u64::from(attachment.size) > MAX_ATTACHMENT_BYTES {
        eprintln!(
            "Skipping download for attachment {}: size {} exceeds limit",
            attachment.id, attachment.size
        );
        return None;
    }

    let extension = attachment_file_extension(&attachment.filename);
    let dir = PathBuf::from(MESSAGE_ATTACHMENTS_DIR).join(message_id);
    if let Err(err) = tokio::fs::create_dir_all(&dir).await {
        eprintln!(
            "Failed to create attachment directory {}: {err}",
            dir.display()
        );
        return None;
    }

    let local_path = dir.join(format!("{}{}", attachment.id, extension));
    let url = if attachment.url.is_empty() {
        attachment.proxy_url.as_str()
    } else {
        attachment.url.as_str()
    };

    match save_url_to_file(url, &local_path).await {
        Ok(()) => Some(local_path.to_string_lossy().into_owned()),
        Err(err) => {
            eprintln!(
                "Failed to download attachment {} from {}: {err}",
                attachment.id, url
            );
            None
        }
    }
}

async fn save_url_to_file(
    url: &str,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(response.error_for_status().unwrap_err().into());
    }
    if response
        .content_length()
        .is_some_and(|len| len > MAX_ATTACHMENT_BYTES)
    {
        return Err("File too large.".into());
    }
    let content = response.bytes().await?;
    if content.len() as u64 > MAX_ATTACHMENT_BYTES {
        return Err("File too large.".into());
    }
    let mut file = tokio::fs::File::create(file_path).await?;
    file.write_all(&content).await?;
    Ok(())
}
