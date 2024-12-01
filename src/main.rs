#![allow(unused)]

mod application;
mod domain;
mod infrastructure;

use application::{config::AppConfig, create_router};
use infrastructure::repository::mongodb_repo::MongodbRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load()?;
    let db_client = MongodbRepository::new().await?;
    let app_state = application::AppState::new(db_client);

    let app = create_router(app_state);

    println!("Server is running on {}...", config.app_addr);
    axum::Server::bind(&config.app_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
