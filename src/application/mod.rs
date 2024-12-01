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
use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub ads_service: Arc<AdsService>,
    pub profile_service: Arc<ProfileService>,
    pub user_service: Arc<UserService>
}

impl AppState {
    pub fn new(db_client: MongodbRepository) -> Self {
        AppState {
            ads_service: Arc::new(AdsService::new(AdsRepository::new(db_client))),
            profile_service: Arc::new(ProfileService::new()),
            user_service: Arc::new(UserService::new())
        }
    }
}

pub fn create_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/v1", axum::routing::get(ads_handler_v1))
        .route("/v2", axum::routing::get(ads_handler_v2))
        .route("/v2/login", axum::routing::post(login))
        .route("/v2/signup", axum::routing::post(signup))
        .route("/v2/ads", axum::routing::post(create_ads_handler_v2))
        .route("/v2/ads", axum::routing::delete(delete_ads_handler_v2))
        .route("/v2/profile", axum::routing::get(profile_handler))
        .layer(Extension(app_state))
        .layer(ServiceBuilder::new().layer(cors).into_inner())
}
