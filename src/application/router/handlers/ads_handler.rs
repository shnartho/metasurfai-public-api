use crate::domain::model::ads::Ads;
use crate::domain::service::ads_service::AdsService;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn ads_handler_v1() -> impl IntoResponse {
    let ads_service = AdsService::new();
    match ads_service.test_get_ads().await {
        Ok(ads) => {
            let ads_json = serde_json::to_string(&ads).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(ads_json))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to get ads"))
            .unwrap(),
    }
}

pub async fn ads_handler_v2() -> impl IntoResponse {
    let ads_service = AdsService::new();
    match ads_service.get_ads().await {
        Ok(ads) => {
            let ads_json = serde_json::to_string(&ads).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(ads_json))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to get ads"))
            .unwrap(),
    }
}

pub async fn create_ads_handler_v2(Json(payload): Json<Ads>) -> impl IntoResponse {
    let ads_service = AdsService::new();
    match ads_service.create_ads(payload).await {
        Ok(ads) => {
            let ads_json = serde_json::to_string(&ads).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(ads_json))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to get ads"))
            .unwrap(),
    }
}

#[derive(serde::Deserialize)]
pub struct DeleteAdsRequest {
    pub id: String,
}

pub async fn delete_ads_handler_v2(Json(payload): Json<DeleteAdsRequest>) -> impl IntoResponse {
    let ads_service = AdsService::new();
    match ads_service.delete_ads(payload.id).await {
        Ok(ads) => {
            let ads_json = serde_json::to_string(&ads).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(ads_json))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to get ads"))
            .unwrap(),
    }
}
