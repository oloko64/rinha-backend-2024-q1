#![allow(clippy::module_name_repetitions)]

mod database;
mod errors;
mod models;
mod requests;
mod responses;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use database::{AppState, Database};
use routes::{get_transactions, make_transaction};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{env, net::SocketAddr, str::FromStr, sync::Arc};

type DbPool = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let max_connections = env::var("MAX_CONNECTIONS")
        .ok()
        .and_then(|m| m.parse::<u32>().ok())
        .unwrap_or(15);
    println!("Max database connections: {max_connections}");

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(PgConnectOptions::from_str(&env::var("DATABASE_URL")?)?)
        .await?;

    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    let state = Arc::new(AppState {
        db: Database::new(pool),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/clientes/:id/transacoes", post(make_transaction))
        .route("/clientes/:id/extrato", get(get_transactions))
        .with_state(state);

    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    println!("Listening on port {port}");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
