use axum::{
    routing::get,
    Router
};
use sqlx::PgPool;
use crate::handlers::trades;

pub fn routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/trades", get(trades::get).post(trades::create))
        .route("/trades/:trade_id", get(trades::get_by_id).patch(trades::edit).delete(trades::delete))
        .with_state(db_pool)
}