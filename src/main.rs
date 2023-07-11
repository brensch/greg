use axum::{
    handler::{get, post},
    http::StatusCode,
    Router,
    Json,
};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use axum::response::IntoResponse;

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Snake {
    pub id: String,
    pub health: i32,
    pub body: Vec<Coordinate>,
    pub head: Coordinate,
    pub length: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coordinate>,
    pub snakes: Vec<Snake>,
}

#[derive(Serialize, Deserialize)]
pub struct GameStatus {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Snake,
}

#[derive(Serialize, Deserialize)]
pub struct RootResponse {
    pub apiversion: String,
    pub author: String,
    pub color: String,
    pub head: String,
    pub tail: String,
    pub version: String,
}

async fn index() -> Json<RootResponse> {
    let response = RootResponse {
        apiversion: String::from("1"),
        author: String::from("MyUsername"),
        color: String::from("#888888"),
        head: String::from("default"),
        tail: String::from("default"),
        version: String::from("0.0.1-beta"),
    };

    Json(response)
}

async fn ping() -> impl IntoResponse {
    println!("got ping");
    (StatusCode::OK, "{}")
}

async fn start() -> impl IntoResponse {
    (StatusCode::OK, "{}")
}

async fn make_move(Json(game_status): Json<GameStatus>) -> impl IntoResponse {
    // Define your logic to decide the next move
    (StatusCode::OK, "{\"move\":\"up\"}")
}

async fn end() -> impl IntoResponse {
    StatusCode::OK
}

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
