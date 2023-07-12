use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Snake {
    pub id: String,
    pub health: i32,
    pub body: Vec<Coordinate>,
    pub head: Coordinate,
    pub length: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coordinate>,
    pub snakes: Vec<Snake>,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize)]
pub struct MoveResponse {
    pub r#move: String,
}