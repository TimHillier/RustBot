//! # Rust OSRS Wiki API Wrapper
//!
//! A wrapper for the OSRS Wiki API.
//!
//! I'm only temporaraly moving this here. I want to push a new update.
//! This will all be removed and moved back into the runescape utils crate once i'm done with it.
//!
//! ## Features
//!
//! - Get the price of an item
//! - Get the price history of an item
//! - Get the price of an item in a specific time period
//! - Get the price of an item in a specific time period

mod rs_client;
pub use rs_client::{
    RSClient, RSItemPrice, RSItemPriceHistory, RSPrice, RSPriceHistoryMapResponse,
    RSPriceMapResponse, TimeStampValue,
};
