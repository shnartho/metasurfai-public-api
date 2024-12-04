pub mod router;
pub mod config;

use std::sync::Arc;

use crate::application::router::handlers::{
    ads_handler::ads_handler_v1, ads_handler::ads_handler_v2, ads_handler::create_ads_handler_v2,
    ads_handler::delete_ads_handler_v2, auth_handler::login, auth_handler::signup,
    profile_handler::profile_handler,
};
use crate::domain::service::{ads_service::AdsService, profile_service::ProfileService, user_service::UserService};
use crate::infrastructure::repository::ads_repository::AdsRepository;
use crate::infrastructure::repository::mongodb_repo::MongodbRepository;
use crate::infrastructure::repository::user_repository::UserRepository;
use axum::{Extension, Router};
use axum::routing::{get, post, delete};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower::limit::RateLimitLayer;
use tower::buffer::BufferLayer;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError};
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    pub ads_service: Arc<AdsService>,
    pub profile_service: Arc<ProfileService>,
    pub user_service: Arc<UserService>
}

impl AppState {
    pub fn new(db_client: MongodbRepository) -> Self {
        AppState {
            ads_service: Arc::new(AdsService::new(AdsRepository::new(db_client.clone()))),
            profile_service: Arc::new(ProfileService::new(UserRepository::new(db_client.clone()))),
            user_service: Arc::new(UserService::new(UserRepository::new(db_client))),
        }
    }
}

pub fn create_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/v1", get(ads_handler_v1))
        .route("/v2", get(ads_handler_v2))
        .route("/v2/login", post(login))
        .route("/v2/signup", post(signup))
        .route("/v2/ads", post(create_ads_handler_v2))
        .route("/v2/ads", delete(delete_ads_handler_v2))
        .route("/v2/profile", get(profile_handler))
        .layer(Extension(app_state))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
                .layer(cors)
                .into_inner(),
        );

    router
}