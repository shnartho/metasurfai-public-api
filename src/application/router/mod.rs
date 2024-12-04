pub mod handlers;
pub mod middlewares;

use crate::application::router::handlers::{
    ads_handler::ads_handler_v1, ads_handler::ads_handler_v2, ads_handler::create_ads_handler_v2,
    ads_handler::delete_ads_handler_v2, auth_handler::login, auth_handler::signup,
    profile_handler::profile_handler,
};
use crate::application::AppState;
use axum::{Extension, Router};
use axum::routing::{get, post, delete};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower::limit::RateLimitLayer;
use tower::buffer::BufferLayer;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError};
use std::time::Duration;

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