use crate::bot_types::{_Context as Context, Error};
use crate::bot_utils::connect_to_database;
use crate::runescape_utils::rs_client::{RSClient, RSPrice, TimeStampValue};
use chrono::DateTime;
use poise::serenity_prelude as serenity;
use quickchart_rs::QuickchartClient;
use serde_json::json;
use thousands::Separable;

// TODO: Move the image and embed stuff into its own method. :3
/// Checks the Grand Exchange for the price of an item.
#[poise::command(prefix_command, aliases("price", "ge", "rsge", "rsprice"))]
pub async fn grand_exchange(
    ctx: Context<'_>,
    #[description = "The name of the item you want to look up"]
    #[rest]
    mut item: String,
) -> Result<(), Error> {
    let database = connect_to_database().await;
    let alias = sqlx::query!("SELECT item FROM ge_aliases WHERE alias = ?", item)
        .fetch_optional(&database)
        .await?;

    if let Some(alias) = alias
        && alias.item.is_some()
    {
        item = alias.item.unwrap();
    }

    let response = RSClient::new().item_name(item).get_price().await?;
    let item_name_formatted = response
        .item
        .replace(' ', "_")
        .replace("'", "")
        .replace("-", "_");

    let image_url = format!(
        "https://oldschool.runescape.wiki/images/{}.png",
        item_name_formatted
    );

    let embed = serenity::CreateEmbed::default()
        .title("💰 Grand Exchange Price")
        .description(format!("**{}**", response.item))
        .field(
            "💵 Price",
            format!("{} gp", response.price.separate_with_commas()),
            true,
        )
        .field("📊 Volume", response.volume.separate_with_commas(), true)
        .color(0xffd700)
        .footer(serenity::CreateEmbedFooter::new(
            "Old School RuneScape Grand Exchange",
        ))
        .thumbnail(image_url);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Checks the Grand Exchange history of an item for the past 10 days.
#[poise::command(prefix_command, aliases("priceHistory", "ph", "history", "hst"))]
pub async fn grand_exchange_history(
    ctx: Context<'_>,
    #[description = "The name of the item you want to look up"]
    #[rest]
    item: String, // make this optional so that if they just do !hs then it just does the last item.
) -> Result<(), Error> {
    let time_length = 10;
    let response = RSClient::new().item_name(item).get_price_history().await?;
    let item_name_formatted = response
        .item
        .replace(' ', "_")
        .replace("'", "")
        .replace("-", "_");

    let image_url = format!(
        "https://oldschool.runescape.wiki/images/{}.png",
        item_name_formatted
    );

    let chart_data = generate_chart_data(response.history, time_length, response.item.clone());

    let chart_url = QuickchartClient::new()
        .chart(chart_data)
        .version("3".to_string())
        .get_short_url()
        .await?;

    print!("Chart URL: {}", chart_url);

    let embed = serenity::CreateEmbed::default()
        .title("Grand Exchange Price History")
        // .description(format!("**{}**", response.item))
        .description(chart_url.clone())
        .image(chart_url)
        .color(0xffd700)
        .footer(serenity::CreateEmbedFooter::new(
            "Old School RuneScape Grand Exchange",
        ))
        .thumbnail(image_url);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(prefix_command, aliases("ge-alias", "gealias", "gea"))]
pub async fn ge_set_alias(
    ctx: Context<'_>,
    #[description = "The alias you want to set"] alias: String,
    #[description = "The item you want to set the alias for"]
    #[rest]
    item: String,
) -> Result<(), Error> {
    let database = connect_to_database().await;

    sqlx::query!(
        "INSERT INTO ge_aliases (alias, item) VALUES (?, ?) \
         ON CONFLICT(alias) DO UPDATE SET item = excluded.item",
        alias,
        item
    )
    .execute(&database)
    .await
    .expect("Couldn't set GE alias.");

    ctx.reply(format!("GE alias `{}` for `{}` set.", alias, item))
        .await?;

    Ok(())
}

#[poise::command(
    prefix_command,
    aliases("ge-lookup-alias", "lookup", "?", "alias", "what")
)]
pub async fn lookup_alias(
    ctx: Context<'_>,
    #[description = "The alias you want to look up"] alias: String,
) -> Result<(), Error> {
    let database = connect_to_database().await;

    let result = sqlx::query!("SELECT item FROM ge_aliases WHERE alias = ?", alias,)
        .fetch_optional(&database)
        .await
        .expect("Couldn't find GE alias.");

    if let Some(result) = result {
        ctx.reply(format!(
            "GE alias `{}` is `{}`.",
            alias,
            result.item.unwrap()
        ))
        .await?;
    } else {
        ctx.reply(format!("GE alias `{}` not found.", alias))
            .await?;
    }

    Ok(())
}

pub fn generate_chart_data(history: Vec<RSPrice>, time_length: u16, item_name: String) -> String {
    let recent_history: Vec<&RSPrice> = history
        .iter()
        .rev()
        .take(time_length as usize)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let nice_item_name = item_name.replace('_', " ").replace("-", " ");

    // Extract labels (timestamps) and data (prices)
    let labels: Vec<String> = recent_history
        .iter()
        .map(|price| {
            // Format timestamp - handle both string and number formats
            match &price.timestamp {
                TimeStampValue::String(s) => {
                    // Try to format ISO 8601 string to a shorter date format
                    s.split('T').next().unwrap_or(s).to_string()
                }
                TimeStampValue::Number(n) => {
                    format!(
                        "{}",
                        DateTime::from_timestamp_millis(*n as i64)
                            .unwrap()
                            .format("%m-%d")
                    )
                }
            }
        })
        .collect();

    let price_data: Vec<u64> = recent_history.iter().map(|price| price.price).collect();
    let volume_data: Vec<u64> = recent_history.iter().map(|price| price.volume).collect();

    let chart_config = json!({
        "type": "line",
        "data": {
            "labels": labels,
            "datasets": [
                {
                    "label": "Price (gp)",
                    "data": price_data,
                    "borderColor": "rgb(255,215,0)",
                    "backgroundColor": "rgba(255,215,0,0.1)",
                    "yAxisID": "yLeft"
                },
                {
                    "label": "Volume",
                    "data": volume_data,
                    "borderColor": "rgb(75,192,192)",
                    "backgroundColor": "rgba(75,192,192,0.1)",
                    "yAxisID": "yRight"
                }
            ]
        },
        "options": {
            "responsive": true,
            "plugins": {
                "title": {
                    "display": true,
                    "text": format!("{} Price History", nice_item_name),
                },
                "legend": {
                    "display": true
                },
                "tickFormat": {
                    "notation": "compact",
                    "maximumFractionDigits": 2
                }
            },
            "scales": {
                "y": {
                    "grid": {
                        "display": true,
                        "color": "grey",
                    },
                },
                "x": {
                    "grid": {
                        "display": true,
                        "color": "grey",
                    },
                },
                "yLeft": {
                    "type": "linear",
                    "display": true,
                    "position": "left",
                    "stacked": false,
                    "beginAtZero": false
                },
                "yRight": {
                    "type": "linear",
                    "display": true,
                    "position": "right",
                    "beginAtZero": false,
                    "grid": {
                        "drawOnChartArea": false
                    }
                }
            }
        }
    });

    serde_json::to_string(&chart_config).unwrap_or_else(|_| "{}".to_string())
}
