use sqlx::{PgPool, Result};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::{
    TradeRow,
    PositionRow
};

pub (crate) async fn create_position (
    pool: &PgPool,
    trade: &TradeRow
) -> Result<PositionRow, sqlx::Error> {
    let position_type = if trade.trade_type == "BUY" { "LONG" } else { "SHORT" };
    println!("again in the thing...");
    let position = sqlx::query!(
        r#"
            INSERT INTO positions (
                symbol,
                balance,
                is_open,
                position_type,
                entry_price,
                quantity,
                pip_price,
                open_time
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING (
                position_id,
                symbol,
                balance,
                is_open,
                position_type,
                entry_price,
                quantity,
                pip_price,
                open_time
            )
        "#,
        trade.symbol,
        Decimal::from_str("0.00").unwrap(),
        true,
        position_type,
        trade.price,
        trade.quantity,
        trade.pip_price,
        trade.trade_time,
    )
    .fetch_one(pool)
    .await?;

    println!("here is the position id after insert {:?}", position);
    Ok( position )
}

// enum TradeAction {
//     OPEN,
//     ADD,
//     REDUCE,
//     CLOSE
// }

// pub (crate) fn calculate_position_action (
//     position: &PositionRow,
//     trade: &TradeRow
// ) -> TradeAction {
//     TradeAction::ADD
// }

pub (crate) async fn update_positions(
    pool: &PgPool,
    trade: &TradeRow
) -> Result<(), sqlx::Error> {
    let positions = sqlx::query_as!(
        PositionRow,
        "SELECT * FROM positions WHERE symbol = $1 AND is_open = true",
        trade.symbol
    )
    .fetch_all(pool)
    .await?;
    // let total_positions = positions.into_iter().for_each(|p| { println!("a thing! {:?}", p)});
    if positions.is_empty() {
        println!("in the empty");
        create_position(pool, trade)
        .await?;
    } else {
        // calculate_position_action(&positions, &trade);
        positions.into_iter().for_each(|p| { println!("found an existing position for symbol {}, id {}", p.symbol, p.position_id)});
    }
    // .map_err(|e| {});
    Ok(())
}