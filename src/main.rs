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
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type DbPool = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect_with(PgConnectOptions::from_str(&env::var("DATABASE_URL")?)?)
        .await?;

    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    let state = Arc::new(AppState {
        db: Database::new(pool),
    });

    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/clientes/:id/transacoes", post(make_transaction))
        .route("/clientes/:id/extrato", get(get_transactions))
        .layer(middlewares)
        .with_state(state);

    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    info!("Listening on port {}", port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
