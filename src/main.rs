use sqlx::{postgres::PgPoolOptions};
use tokio::net::TcpListener;

mod handlers;
mod models;
mod create_app;

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

    let app = create_app::routes(db_pool);

    axum::serve(listener, app).await.expect("Error serving application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use http_body_util::BodyExt;

    async fn setup_test_db() -> PgPool {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for tests");

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to db")
    }

    #[tokio::test]
    async fn get_trades() {
        let pool = setup_test_db().await;
        let app = create_app::routes(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/trades")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(&body).unwrap();

        assert!(body_str.contains("success"));
    }
}