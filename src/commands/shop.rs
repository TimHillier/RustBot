use crate::bot_types::{_Context as Context, Error};
use crate::bot_utils;
use poise::serenity_prelude as serenity;
use std::fmt::{Display, Formatter, Result as fmtResult};
use crate::bot_utils::{connect_to_database, get_current_bot_id, get_plus_two_received, take_plus_two};
use crate::commands::trade::do_transaction;
use crate::emoji::get_emoji;

static ITEM_COL_WIDTH: usize = 70;
static PRICE_COL_WIDTH: usize = 15;
static SHORT_NAME_COL_WIDTH: usize = 15;
pub struct ItemInfo {
    pub(crate) item_name: String,
    pub(crate) short_name: String,
    pub(crate) price: String,
    pub(crate) description: String,
}

impl Display for ItemInfo {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(
            f,
            "{:<SHORT_NAME_COL_WIDTH$}{:^ITEM_COL_WIDTH$}{:>PRICE_COL_WIDTH$}",
            self.short_name, self.item_name, self.price
        )
    }
}

pub(crate) struct ItemInfoVec(pub Vec<ItemInfo>);
impl Display for ItemInfoVec {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        let mut result = String::new();
        for item in &self.0[0..self.0.len() - 1] {
            result.push_str(&item.to_string());
        }
        write!(f, "{}", result)
    }
}

/// Displays the Shop. Allowing users to buy items
/// Use !shop -> shows items
/// use !shop buy <symbol> -> to buy item with symbol
#[poise::command(prefix_command, aliases("store"), subcommands("buy"))]
pub async fn shop(ctx: Context<'_>) -> Result<(), Error> {
    let shop_items = bot_utils::get_shop_items().await;
    let mut shop_string: String = String::new();
    for (i, value) in shop_items.0.iter().enumerate() {
        shop_string.push_str(&value.to_string());
    }

    let mut shop_item_fields = Vec::new();
    for shop_item in &shop_items.0 {
        shop_item_fields.push((
            format!(
                "{:<SHORT_NAME_COL_WIDTH$}{:^ITEM_COL_WIDTH$}{:>PRICE_COL_WIDTH$}",
                shop_item.short_name, shop_item.item_name, shop_item.price
            ),
            shop_item.description.clone(),
            false,
        ))
    }

    let shop_embed = serenity::CreateEmbed::default()
        .title("Shop")
        .field(
            format!(
                "{:<SHORT_NAME_COL_WIDTH$}{:^ITEM_COL_WIDTH$}{:>PRICE_COL_WIDTH$}",
                "Symbol", "Item", "Price"
            ),
            format!("{:-<75}", ""),
            false,
        )
        .fields(shop_item_fields);

    let shop_message = {
        poise::CreateReply::default()
            .embed(shop_embed.clone())
            .reply(true)
            .ephemeral(true)
    };

    ctx.send(shop_message).await?;
    Ok(())
}

/// Allows the user to buy an item from the shop.
#[poise::command(prefix_command)]
pub async fn buy(
    ctx: Context<'_>,
    #[description = "The symbol of the item you want"] symbol: String,
) -> Result<(), Error> {
    let database = connect_to_database().await;
    let selected_item = sqlx::query!(
        "SELECT * FROM shop_items WHERE short_name = ?", symbol,
    ).fetch_one(&database)
    .await?;

    if get_plus_two_received(ctx.author().id.to_string()).await.unwrap() < selected_item.price {
        ctx.reply(format!("Not Enough {}", get_emoji("plus_two"))).await?;
        return Ok(());
    }

    let current_bot_id = get_current_bot_id().await.to_string();
    do_transaction(&ctx.author().id.to_string(), &current_bot_id , selected_item.price as i16).await;
    update_shop_count(selected_item.short_name, 1, 1).await;
    ctx.reply("Bought").await?;

    Ok(())
}

pub async fn update_shop_count(item_name: String, current_increase: i16, total_increase: i16) {
    let database = connect_to_database().await;
    let _update_db = sqlx::query!(
        "UPDATE shop_items SET current_amount = current_amount + ?, total_amount = total_amount + ? WHERE short_name = ?",
        current_increase,
        total_increase,
        item_name
    ).execute(&database).await.expect("Failed to update shop count");


}

#[poise::command(prefix_command, aliases("count", "getCount"))]
pub async fn item_count(
    ctx: Context<'_>,
    #[description = "The symbol of the item you want"] symbol: String,
) -> Result<(), Error> {
    let count = bot_utils::get_count(&symbol).await;
    ctx.reply(format!("current {} count: {}", symbol, count)).await?;
    Ok(())
}
