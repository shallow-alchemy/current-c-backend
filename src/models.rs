use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct TradeRow {
    pub trade_id: i32,
    pub symbol: String,
    pub account_balance: Decimal,
    pub trade_type: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub pip_price: Decimal,
    pub spread: Decimal,
    pub trade_time: DateTime<Utc>,
    pub notes: Option<String>
}

#[derive(Deserialize)]
pub struct CreateTradeReq {
    pub symbol: String
}

#[derive(Serialize)]
pub struct CreateTradeRow {
    pub trade_id: i32,
}

#[derive(Deserialize)]
pub struct EditTradeReq {
    pub symbol: Option<String>
}