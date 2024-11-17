mod application;
mod domain;
mod infrastructure;

use application::create_router;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let app = create_router();
    let addr = "[::]:8080".parse().unwrap();

    println!("Server is running on port 8080...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}
