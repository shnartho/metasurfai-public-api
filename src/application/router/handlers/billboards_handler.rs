use crate::application::AppState;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;

pub async fn get_billboards_handler(Extension(app_state): Extension<AppState>) -> impl IntoResponse {
    let billboards_service = app_state.billboard_service.clone();
    match billboards_service.get_billboards().await {
        Ok(billboards) => {
            let billboards_json = serde_json::to_string(&billboards).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(billboards_json))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to get billboards"))
            .unwrap(),
    }
}