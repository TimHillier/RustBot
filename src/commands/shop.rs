use crate::bot_utils;
use crate::bot_types::{Error, _Context as Context};
use std::fmt::{Display, Formatter, Result as fmtResult};
use poise::serenity_prelude as serenity;


static ITEM_COL_WIDTH:usize = 70;
static PRICE_COL_WIDTH:usize = 15;
static AMOUNT_COL_WIDTH:usize = 15;
pub struct ItemInfo {
    pub(crate) item_name: String,
    pub(crate) short_name: String,
    pub(crate) price: String,
    pub(crate) amount: String,
    pub(crate) description: String,
}

impl Display for ItemInfo {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{:<AMOUNT_COL_WIDTH$}{:^ITEM_COL_WIDTH$}{:>PRICE_COL_WIDTH$}", self.amount, self.item_name, self.price)
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
/// use !shop buy <n> -> to buy item n
#[poise::command(prefix_command)]
pub async fn shop(ctx: Context<'_>) -> Result<(), Error> {
    let shop_items = bot_utils::get_shop_items().await;
    let mut shop_string: String = String::new();
    for (i, value) in shop_items.0.iter().enumerate() {
        shop_string.push_str(&value.to_string());
    }

    let mut shop_item_fields = Vec::new();
    for shop_item in &shop_items.0 {
        shop_item_fields.push((format!("{:<AMOUNT_COL_WIDTH$}{:^ITEM_COL_WIDTH$}{:>PRICE_COL_WIDTH$}",shop_item.amount, shop_item.item_name, shop_item.price), shop_item.description.clone(), false))
    }

    let shop_embed = serenity::CreateEmbed::default()
        .title("Shop")
        .field(format!("{:<AMOUNT_COL_WIDTH$}{:^ITEM_COL_WIDTH$}{:>PRICE_COL_WIDTH$}","Stock", "Item", "Price"), format!("{:-<75}", ""), false)
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