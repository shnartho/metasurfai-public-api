use axum::{
    routing::{get, Router},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "OK".into(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/v1", get(hello));

    let addr = "[::]:8080".parse().unwrap();
    println!("Listening on {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
