mod application;
mod domain;
mod infrastructure;

use application::create_router;
use application::router::middlewares::auth::Auth;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let auth = Arc::new(Auth);
    let app = create_router(auth);
    let addr = "[::]:8080".parse().unwrap();
    println!("Server is running on port 8080...");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::response::Response;

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
            .body(Body::from(r#"{"username": "nayem", "password": "123"}"#))
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
*/
