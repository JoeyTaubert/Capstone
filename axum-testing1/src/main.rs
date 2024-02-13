use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio;

// Reference: https://github.com/programatik29/axum-tutorial/blob/master/tutorial/01-introduction.md
// Axum docs: https://docs.rs/axum/latest/axum/#example


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

