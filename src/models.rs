use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub (crate) struct TradeRow {
    pub (crate) trade_id: i32,
    pub (crate) symbol: String,
    pub (crate) account_balance: Decimal,
    pub (crate) trade_type: String,
    pub (crate) price: Decimal,
    pub (crate) quantity: Decimal,
    pub (crate) pip_price: Decimal,
    pub (crate) spread: Decimal,
    pub (crate) trade_time: DateTime<Utc>,
    pub (crate) notes: Option<String>
}

#[derive(Serialize, Debug)]
pub (crate) struct PositionRow {
    pub (crate) position_id: i32,
    pub (crate) symbol: String,
    pub (crate) balance: Decimal,
    pub (crate) is_open: bool,
    pub (crate) position_type: String,
    pub (crate) entry_price: Decimal,
    pub (crate) close_price: Option<Decimal>,
    pub (crate) quantity: Decimal,
    pub (crate) pip_price: Decimal,
    pub (crate) pip_diff: Option<Decimal>,
    pub (crate) profit_loss: Option<Decimal>,
    pub (crate) win_loss: Option<String>,
    pub (crate) open_time: DateTime<Utc>,
    pub (crate) close_time: Option<DateTime<Utc>>,
    pub (crate) notes: Option<String>
}

pub (crate) struct PositionTradeRow {
    pub (crate) position_id: i32,
    pub (crate) trade_id: i32,
    pub (crate) quantity_allocated: Decimal,
    pub (crate) trade_action: String,
}

#[derive(Deserialize)]
pub (crate) struct CreateTradeReq {
    pub (crate) symbol: String,
    pub (crate) account_balance: Decimal,
    pub (crate) trade_type: String,
    pub (crate) price: Decimal,
    pub (crate) quantity: Decimal,
    pub (crate) pip_price: Decimal,
    pub (crate) spread: Decimal,
    pub (crate) trade_time: DateTime<Utc>,
    pub (crate) notes: Option<String>
}

#[derive(Deserialize)]
pub (crate) struct EditTradeReq {
    pub (crate) symbol: Option<String>
}