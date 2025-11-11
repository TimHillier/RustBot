use crate::UserInfo;
use rand::Rng;
use serde::Deserialize;
use serenity::{Result as SerenityResult, model::channel::Message};
use sqlx::{Pool, Sqlite};
use std::fs;
use toml;
use crate::commands::shop::ItemInfo;

#[derive(Debug, Deserialize)]
struct SecretsToml {
    #[allow(dead_code)]
    discord_token: String,
    environment: String,
}

pub fn get_toml() -> String {
    let toml_str = fs::read_to_string("data/Secrets.toml").expect("Failed to read TOML");
    return toml_str;
}
pub fn get_secret() -> String {
    let toml_str = get_toml();
    let secrets_toml: SecretsToml = toml::from_str(&toml_str).expect("Failed to decode toml");
    return secrets_toml.discord_token;
}

pub fn get_env() -> String {
    let toml_str = get_toml();
    let secrets_toml: SecretsToml = toml::from_str(&toml_str).expect("Failed to decode toml");
    let environment = secrets_toml.environment;
    if environment.is_empty() {
        return String::from("testing");
    }
    return environment;
}

pub fn get_random_bool(prob: f64) -> bool {
    let mut rng = rand::thread_rng();
    return rng.gen_bool(prob);
}

#[allow(dead_code)]
pub fn get_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..999);
}

// A connection to the database.
pub async fn connect_to_database() -> Pool<Sqlite> {
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("data/rustbot.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't Connect to database.");

    return database;
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

    return plus_2_amount.plus_two_received;
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
Directly give the user_id plus 2's
**/
pub async fn give_minus_two(user_id: &str, amount_given: i16) {
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET minus_two_received = minus_two_received + ? WHERE user_id = ?",
        amount_given,
        user_id
    )
    .execute(&database)
    .await
    .expect("Couldn't give minus two");
}

/**
Get the current amount of minus 2's the user has.
**/
pub async fn get_minus_two_received(user_id: &str) {
    let database = connect_to_database().await;
    let plus_2_amount = sqlx::query!(
        "SELECT minus_two_received FROM user WHERE user_id = ?",
        user_id
    )
    .fetch_all(&database)
    .await
    .unwrap();
}

/**
Directly take the user_id plus 2's
**/
pub async fn take_minus_two(user_id: &str, amount_taken: i16) {
    let database = connect_to_database().await;
    sqlx::query!(
        "UPDATE user SET minus_two_received = minus_two_received - ? WHERE user_id = ?",
        amount_taken,
        user_id
    )
    .execute(&database)
    .await
    .expect("Couldn't take minus two");
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

    return UserInfo {
        user_name: user.user_name,
        score: user.score.unwrap(),
    };
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

    return result.score.unwrap();
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

    return user_vector;
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
        "SELECT item_name, price, amount, short_name, description FROM shop_items ORDER BY price DESC",
    )
    .fetch_all(&database)
    .await
    .unwrap();

    let mut item_vector = crate::commands::shop::ItemInfoVec(vec![]);
    for item in shop_items.iter() {
        let temp_item = ItemInfo {
            item_name: item.item_name.to_string(),
            short_name: item.short_name.to_string(),
            amount: item.amount.to_string(),
            price: item.price.to_string(),
            description: item.description.to_string(),
        };
        item_vector.0.push(temp_item)
    }
    return item_vector;
}
