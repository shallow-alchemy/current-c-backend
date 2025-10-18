use axum::{
    extract::{Path, State},
    http::StatusCode, Json,
};
use serde_json::json;
use sqlx::PgPool;
use crate::models::{
    TradeRow,
    CreateTradeReq,
    CreateTradeRow,
    EditTradeReq
};

pub async fn get(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(
        TradeRow,
        "SELECT * FROM trades ORDER BY trade_id"
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string()}).to_string(),
        )
    })?;

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
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "success": false, "message": e.to_string() }).to_string(),
                )
            })?;
        
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
        CreateTradeRow,
        "INSERT INTO trades (symbol) VALUES ($1) RETURNING trade_id",
        trade.symbol
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode:: INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::CREATED,
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
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok(( StatusCode::OK, json!({ "success": true}).to_string() ))
}

pub async fn delete(
    State(pg_pool): State<PgPool>,
    Path(trade_id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM trades WHERE trade_id = $1", trade_id)
    .execute(&pg_pool)
    .await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string()}).to_string()
        )
    })?;

    Ok(( StatusCode::OK, json!({ "success": true}).to_string() ))
}
