mod db;
use db::PrismaClient;
mod user;
use user::*;
mod errors;
mod utils;

use anyhow::Result;
use once_cell::sync::OnceCell;
use salvo::{jwt_auth::HeaderFinder, prelude::*};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

static DB: OnceCell<PrismaClient> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Initializing prisma client");
    let client = db::new_client().await?;
    DB.set(client).unwrap();

    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(TOKEN_SALT.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    let router = Router::with_path("api");

    let user_router = Router::with_path("user")
        .push(Router::with_path("register").post(user_register))
        .push(Router::with_path("login").post(user_login));
    let router = router.push(user_router);

    let company_router =
        Router::with_path("company").push(Router::with_path("register").post(company_register));
    let router = router.push(company_router);

    let hooped_router = Router::new().hoop(auth_handler);

    let router =
        router.push(hooped_router.push(Router::with_path("user/details/<id:num>").get(user_details)));

    Server::new(TcpListener::new("127.0.0.1:7878").bind().await)
        .serve(router)
        .await;
    Ok(())
}
