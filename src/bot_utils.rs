use std::fs;
use rand::Rng;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use toml;
use serenity::{
    model::{channel::Message},
    Result as SerenityResult,
};

#[derive(Debug, Deserialize)]
struct SecretsToml {
    #[allow(dead_code)]
    discord_token: String,
    environment: String
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

pub async fn score_update(user_id: &str, points:i8) {
    let database = connect_to_database().await;

    sqlx::query!(
        "UPDATE user SET score = score + ? WHERE user_id = ?",
        points,
        user_id
    )
        .execute(&database)
        .await.expect("Couldn't increase users score.");
}

pub async fn gave_plus_two(user_id: &str, removed:bool) {
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
        .await.expect("Couldn't give plus two");
}
pub async fn gave_minus_two(user_id: &str, removed:bool) {
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
        .await.expect("Couldn't give minus two");
}
pub async fn received_plus_two(user_id: &str, removed:bool) {
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
        .await.unwrap();
}
pub async fn received_minus_two(user_id: &str, removed:bool) {
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
        .await.unwrap();
}

pub async fn plus_two(giver_id: &str, received_id: &str, removed: bool) {
    gave_plus_two(giver_id, removed).await;
    received_plus_two(received_id, removed).await;

}

pub async fn minus_two(giver_id: &str, received_id: &str, removed: bool) {
    gave_minus_two(giver_id, removed).await;
    received_minus_two(received_id, removed).await;
}


pub async fn score_insert(user_id: &str, user_name:&str) {
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

/// Checks that a message got sent correctly.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

