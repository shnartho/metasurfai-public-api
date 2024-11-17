pub mod router;

use crate::application::router::handlers::{
    ads_handler::ads_handler_v1, ads_handler::ads_handler_v2, ads_handler::create_ads_handler_v2,
    ads_handler::delete_ads_handler_v2, auth_handler::login, auth_handler::signup,
    profile_handler::profile_handler,
};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub fn create_router() -> Router {
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
        .layer(ServiceBuilder::new().layer(cors).into_inner())
}
