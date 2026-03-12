use crate::bot_types::{_Context as Context, Error};
use image;
use poise::serenity_prelude as serenity;
use serenity::all::Message;
use serenity::builder::CreateMessage;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

const ROOT: &str = "data/discord_images";
const ORIGINAL_FOLDER: &str = "original";
const ROTATE_FOLDER: &str = "rotated";

#[poise::command(prefix_command)]
pub async fn rotate(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.channel_id().message(&ctx.http(), ctx.id()).await?;
    // rotate_image(msg.clone()).await;

    if msg.referenced_message.is_none() {
        ctx.reply("Nothing to rotate...")
            .await
            .expect("Or maybe there was?");
    }

    let ref_msg = msg.referenced_message.clone().unwrap();

    if !matches!(
        ref_msg
            .clone()
            .attachments
            .first()
            .and_then(|a| a.content_type.as_deref()),
        Some("image/png" | "image/jpeg")
    ) {
        ctx.reply("That's not an image")
            .await
            .expect("Or maybe it was?");
    }
    make_folders();

    let attachments_link = ref_msg.attachments.first().unwrap().proxy_url.clone();

    let content_type = ref_msg
        .attachments
        .first()
        .unwrap()
        .content_type
        .clone()
        .unwrap();

    let file_name = generate_file_name(ref_msg.id.to_string(), &content_type);

    let original_file_path = PathBuf::from(ROOT).join(ORIGINAL_FOLDER).join(&file_name);
    get_and_save_picture(&attachments_link, &original_file_path).await?;

    let rotate_file_path = PathBuf::from(ROOT).join(ROTATE_FOLDER).join(&file_name);
    rotate_image_and_save(&original_file_path, &rotate_file_path).await;

    let attachment = serenity::CreateAttachment::path(&rotate_file_path).await?;
    ctx.send(poise::CreateReply::default().attachment(attachment))
        .await?;

    Ok(())
}

#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR")]
pub async fn check_folder(ctx: Context<'_>) -> Result<(), Error> {
    make_folders();
    ctx.reply(format!("Folder Status: {}", check_folders()))
        .await?;
    Ok(())
}

/**
Returns file name usually message id + file extension.
**/
fn generate_file_name(file_name: String, content_type: &str) -> String {
    let file_extension = match content_type {
        "image/png" => ".png",
        "image/jpeg" => ".jpg",
        _ => ".png",
    };
    file_name + file_extension
}

/**
Checks if paths exist, and creates them if they don't.
**/
fn make_folders() {
    let original_path = PathBuf::from(ROOT).join(ORIGINAL_FOLDER);
    let rotate_path = PathBuf::from(ROOT).join(ROTATE_FOLDER);

    if !Path::new(ROOT).exists() {
        std::fs::create_dir_all(ROOT).expect("Failed to created directory");
    }

    if !Path::new(&original_path).exists() {
        std::fs::create_dir_all(original_path).expect("Failed to created directory");
    }

    if !Path::new(&rotate_path).exists() {
        std::fs::create_dir_all(rotate_path).expect("Failed to created directory");
    }
}

/**
Checks to see if the folders are make
**/
fn check_folders() -> bool {
    let original_path = PathBuf::from(ROOT).join(ORIGINAL_FOLDER);
    let rotate_path = PathBuf::from(ROOT).join(ROTATE_FOLDER);
    let mut status = true;

    if !Path::new(ROOT).exists() {
        status = false;
    }

    if !Path::new(&original_path).exists() {
        status = false;
    }

    if !Path::new(&rotate_path).exists() {
        status = false;
    }
    status
}

/**
Downloads image from url and saves it to file.
**/
async fn get_and_save_picture(link: &str, file_path: &Path) -> Result<bool, Error> {
    let response = reqwest::get(link).await?;
    if !response.status().is_success() {
        return Err(Error::from(response.error_for_status().unwrap_err()));
    }
    const MAX_SIZE: u64 = 50_000_000;
    if response.content_length().is_none_or(|len| len > MAX_SIZE) {
        return Err(Error::from("File to large."));
    }
    let mut file = tokio::fs::File::create(file_path).await?;
    let content = response.bytes().await?;
    file.write_all(&content).await?;
    Ok(true)
}

/**
Rotate and save the image.
**/
async fn rotate_image_and_save(file_path: &Path, save_path: &Path) {
    let img = image::open(file_path).unwrap();
    //Todo: Update this to take rotation.
    let rot_img = img.rotate180();
    rot_img.save(save_path).expect("Error Saving Image");
}

pub async fn rotate_image_directly(
    http: &serenity::Http,
    channel_id: serenity::ChannelId,
    msg: &Message,
) -> Result<(), Error> {
    if !matches!(
        msg.attachments
            .first()
            .and_then(|a| a.content_type.as_deref()),
        Some("image/png" | "image/jpeg")
    ) {
        return Err(Error::from("Not a valid image"));
    }
    make_folders();
    let attachments_link = msg.attachments.first().unwrap().proxy_url.clone();
    let content_type = msg
        .attachments
        .first()
        .unwrap()
        .content_type
        .clone()
        .unwrap();

    let file_name = generate_file_name(msg.id.to_string(), &content_type);

    let original_file_path = PathBuf::from(ROOT).join(ORIGINAL_FOLDER).join(&file_name);
    get_and_save_picture(&attachments_link, &original_file_path).await?;

    let rotate_file_path = PathBuf::from(ROOT).join(ROTATE_FOLDER).join(&file_name);
    rotate_image_and_save(&original_file_path, &rotate_file_path).await;

    let attachment = serenity::CreateAttachment::path(&rotate_file_path).await?;
    let builder = CreateMessage::new().add_file(attachment);
    channel_id.send_message(http, builder).await?;

    Ok(())
}
