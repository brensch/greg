use axum::{
    handler::{get, post},
    Router,
};
use std::net::SocketAddr;
use crate::handlers::{index, ping, start, make_move, end};

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    println!("Starting application");

    let port = std::env::var("PORT").unwrap_or_else(|_| {
        println!("No PORT environment variable detected, falling back to 8000");
        String::from("8000")
    });

    let parsed_port = port.parse().unwrap_or_else(|_| {
        println!("Failed to parse PORT environment variable, falling back to 8000");
        8000
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], parsed_port));

    println!("Server will bind to: {}", addr);

    let app = Router::new()
    .route("/", get(index))
    .route("/ping", post(ping))
    .route("/start", post(start))
    .route("/move", post(make_move)) 
    .route("/end", post(end));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    tokio::select! {
        _ = server => {
            println!("Server error.");
        },
        _ = shutdown_signal => {
            println!("Shutting down.");
        },
    }
}
