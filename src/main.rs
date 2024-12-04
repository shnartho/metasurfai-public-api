mod application;
mod domain;
mod infrastructure;

use application::config::AppConfig;
use application::router::create_router;
use infrastructure::repository::mongodb_repo::MongodbRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load()?;
    let db_client = MongodbRepository::new(config.mongo_username, config.mongo_password).await?;
    let app_state = application::AppState::new(db_client);
    let router = create_router(app_state);

    println!("Server is running on {}...", config.app_addr);
    axum::Server::bind(&config.app_addr.parse().unwrap())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
