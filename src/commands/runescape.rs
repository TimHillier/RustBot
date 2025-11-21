use crate::bot_types::{_Context as Context, Error};
// use crate::thousands::Separable;
use poise::serenity_prelude as serenity;
use rust_osrs_wiki_api_wrapper::RSClient;
use thousands::Separable;

/// Checks the Grand Exchange item price of an item.
#[poise::command(prefix_command, aliases("price", "ge", "rsge", "rsprice"))]
pub async fn grand_exchange(
    ctx: Context<'_>,
    #[description = "The name of the item you want to look up"]
    #[rest]
    item: String,
) -> Result<(), Error> {
    let response = RSClient::new().get_ge_price(item).await?;
    ctx.reply(format!(
        "{}\n Price: {}gp\n Volume: {}",
        response.item_name,
        response.price.separate_with_commas(),
        response.volume.separate_with_commas()
    ))
    .await?;

    Ok(())
}

// /// Returns information about an item from the osrs wiki.
// #[poise::command(prefix_command, aliases("rswiki", "rsw", "rs"))]
// pub async fn wiki_check(ctx: crate::bot_types::_Context<'_>, #[description="The name of the item you want to look up"]) -> Result<(), Error> {
//
// }
