pub mod router;

use crate::application::router::handlers::{
    ads_handler::ads_handler_v1, ads_handler::ads_handler_v2, auth_handler::login,
    profile_handler::profile_handler, ads_handler::create_ads_handler_v2, ads_handler::delete_ads_handler_v2,
};
use crate::application::router::middlewares::auth::Auth;
use axum::extract::Extension;
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub fn create_router(auth: Arc<Auth>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/v1", axum::routing::get(ads_handler_v1))
        .route("/v2", axum::routing::get(ads_handler_v2))
        .route("/v2/createOneAds", axum::routing::post(create_ads_handler_v2))
        .route("/v2/deleteOneAds", axum::routing::delete(delete_ads_handler_v2))
        .route("/v1/login", axum::routing::post(login))
        .route(
            "/v1/profile",
            axum::routing::get(profile_handler).layer(Extension(auth.clone())),
        )
        .layer(
            ServiceBuilder::new()
                .layer(Extension(auth))
                .layer(cors)
                .into_inner(),
        )
}
