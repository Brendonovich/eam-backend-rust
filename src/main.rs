mod db;
use std::net::SocketAddr;

use db::PrismaClient;
mod user;
use user::*;
mod assets;
use assets::*;
mod maintainance_request;
use maintainance_request::*;
mod errors;
mod utils;

use anyhow::Result;
use axum::{
    routing::{get, post, put},
    Router,
};
use once_cell::sync::OnceCell;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

static DB: OnceCell<PrismaClient> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Initializing prisma client");
    let client = db::new_client().await?;
    DB.set(client).unwrap();

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let user_router = Router::new()
        .route("/register", post(user_register))
        .route("/login", post(user_login))
        .route("/details/:id", get(user_details))
        .route("/update/:id", put(update_user));

    let company_router = Router::new().route("/register", post(company_register));

    let asset_router = Router::new()
        .route("/", post(create_asset))
        .route("/:id", get(get_asset))
        .route("/location", post(create_location))
        .route("/location/:id", get(get_location))
        .route("/location/nested", get(get_nested_location))
        .route("/status", post(create_status))
        .route("/status/:id", get(get_status));

    let mr_router = Router::new()
        .route("/", post(create_mr))
        .route("/:id", get(get_mr))
        .route("/status", post(create_mr_status))
        .route("/status/:id", post(get_mr_status))
        .route("/priority", post(create_mr_priority))
        .route("/priority/:id", post(get_mr_priority))
        .route("/category", post(create_mr_category))
        .route("/category/:id", post(get_mr_category))
        .route("/failure_impact", post(create_mr_failure_impact))
        .route("/failure_impact/:id", post(get_mr_failure_impact))
        .route("/failure_mode", post(create_mr_failure_mode))
        .route("/failure_mode/:id", post(get_mr_failure_mode));

    let api_routes = Router::new()
        .nest("/user", user_router)
        .nest("/company", company_router)
        .nest("/asset", asset_router)
        .nest("/mr", mr_router);

    let router = Router::new().nest("/api", api_routes).layer(cors);
    tracing::debug!("{:?}", router);
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
