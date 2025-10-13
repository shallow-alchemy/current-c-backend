use axum::{
    extract::{ Path, State},
    http::StatusCode,
    routing::{ get },
    Json, Router
};

use serde::{ Serialize, Deserialize};
use serde_json::json;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address: String = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Can't connect to database");

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async { "Hello world!"}))
        .route("/trades", get(get_trades).post(create_trade))
        .route("/trades/:trade_id", get(get_trade_by_id).patch(edit_trade).delete(delete_trade))
        .with_state(db_pool);

    axum::serve(listener, app).await.expect("Error serving application");
}

#[derive(Serialize)]
struct TradeRow {
    trade_id: i32,
    symbol: String
}

async fn get_trades(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(TradeRow, "SELECT * FROM trades ORDER BY trade_id")
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

#[derive(Deserialize)]
struct GetTradeByIdReq {
    trade_id: i32
}


async fn get_trade_by_id(
    State(pg_pool): State<PgPool>,
    Path(trade_id_str): Path<String>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("hello there");
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

#[derive(Deserialize)]
struct CreateTradeReq {
    symbol: String
}

#[derive(Serialize)]
struct CreateTradeRow {
    trade_id: i32,
}

async fn create_trade(
    State(pg_pool): State<PgPool>,
    Json(trade): Json<CreateTradeReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("hello there again");
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

#[derive(Deserialize)]
struct EditTradeReq {
    symbol: Option<String>
}

async fn edit_trade(
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

async fn delete_trade(
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
