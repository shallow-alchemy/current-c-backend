use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TradeRow {
    pub trade_id: i32,
    pub symbol: String
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