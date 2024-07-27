mod application;
mod domain;
mod infrastructure;

use axum::{
    routing::{get, post},
    Router,
    extract::Extension,
};
use tower::ServiceBuilder;
use application::router::handlers::auth_handler::login;
use application::router::middlewares::auth::Auth;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // Shared state
    let auth = Arc::new(Auth);

    let app = Router::new()
        .route("/v1/login", post(login))
        .route("/v1/protected", get(protected).layer(Extension(auth.clone())))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(auth))
                .into_inner()
        );
        
    axum::Server::bind(&"[::]:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
}

async fn protected() -> impl axum::response::IntoResponse {
    "You are authorized"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::response::Response;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_login_success_200() {
        let auth = Arc::new(Auth);
        let app = Router::new()
        .route("/v1/login", post(login))
        .layer(
            ServiceBuilder::new() 
            .layer(Extension(auth))
            .into_inner()
        );

        let request = Request::builder()
            .uri("/v1/login")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username": "jemal", "password": "muntasir"}"#))
            .unwrap();

        let response: Response = app.oneshot(request).await.unwrap();
        println!("{:?}", response);
        assert_eq!(response.status(), StatusCode::OK)
    } 

    #[tokio::test]
    async fn test_login_failed_401() {
        let auth = Arc::new(Auth);
        let app = Router::new()
        .route("/v1/login", post(login))
        .layer(
            ServiceBuilder::new() 
            .layer(Extension(auth))
            .into_inner()
        );

        let request = Request::builder()
            .uri("/v1/login")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username": "test1", "password": "test2"}"#))
            .unwrap();

        let response: Response = app.oneshot(request).await.unwrap();
        println!("{:?}", response);
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED)
    }   
}