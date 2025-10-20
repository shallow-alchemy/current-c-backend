use axum::{
    extract::{ Path, State },
    http::StatusCode, Json,
};
use serde_json::json;
use sqlx::PgPool;
use crate::models::{
    TradeRow,
    CreateTradeReq,
    EditTradeReq
};
use crate::services::{ positions, api };

pub async fn get(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(
        TradeRow,
        "SELECT * FROM trades ORDER BY trade_id"
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(api::endpoint_error)?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": rows }).to_string()
    ))
}

pub async fn get_by_id(
    State(pg_pool): State<PgPool>,
    Path(trade_id_str): Path<String>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    match trade_id_str.parse::<i32>() {
        Err(_) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": format!("Invalid Trade Id: {}. Must be an integer.", trade_id_str)}).to_string()
            ))
        }
        
        Ok(trade_id_int) => {
            let trades = sqlx::query_as!(
                TradeRow,
                "SELECT * FROM trades WHERE trade_id = $1",
                trade_id_int
            )
            .fetch_all(&pg_pool)
            .await
            .map_err(api::endpoint_error)?;

            Ok((
                StatusCode::OK,
                json!({ "success": true, "data": trades }).to_string()
            ))
        }
    }
}

pub async fn create(
    State(pg_pool): State<PgPool>,
    Json(trade): Json<CreateTradeReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        TradeRow,
        r#"
        INSERT INTO trades (
            symbol,
            account_balance,
            trade_type,
            price,
            quantity,
            pip_price,
            spread,
            trade_time,
            notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING trade_id, symbol, account_balance, trade_type, price, quantity, pip_price, spread, trade_time, notes
        "#,
        trade.symbol,
        trade.account_balance,
        trade.trade_type,
        trade.price,
        trade.quantity,
        trade.pip_price,
        trade.spread,
        trade.trade_time,
        trade.notes
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode:: INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;
    positions::update_positions(&pg_pool, &row)
    .await
    .map_err(api::endpoint_error)?;

    Ok((
        StatusCode::CREATED,
        json!({ "success": true, "data": row }).to_string()
    ))
}

pub async fn edit(
    State(pg_pool): State<PgPool>,
    Path(trade_id): Path<i32>,
    Json(trade): Json<EditTradeReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "UPDATE trades SET symbol = $2 WHERE trade_id = $1",
        trade_id,
        trade.symbol
    )
    .execute(&pg_pool)
    .await
    .map_err(api::endpoint_error)?;

    Ok(( StatusCode::OK, json!({ "success": true}).to_string() ))
}

pub async fn delete(
    State(pg_pool): State<PgPool>,
    Path(trade_id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM trades WHERE trade_id = $1", trade_id)
    .execute(&pg_pool)
    .await
    .map_err(api::endpoint_error)?;

    Ok(( StatusCode::OK, json!({ "success": true}).to_string() ))
}
