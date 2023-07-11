use axum::{
    handler::{get, post},
    http::StatusCode,
    Router,
    Json,
};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
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



// Your GameStatus and other struct definitions here.

async fn index() -> &'static str {
    "Hello, Battlesnake!"
}

async fn ping() -> impl IntoResponse {
    (StatusCode::OK, "{}")
}

async fn start() -> impl IntoResponse {
    (StatusCode::OK, "{\"color\":\"#FF0000\",\"headType\":\"fang\",\"tailType\":\"bolt\"}")
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
    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("8000"));
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));

    let app = Router::new()
        .route("/", get(index))
        .route("/ping", post(ping))
        .route("/start", post(start))
        .route("/move", post(make_move))
        .route("/end", post(end));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}