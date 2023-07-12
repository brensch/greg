use axum::{Json, http::StatusCode};
use crate::models::{RootResponse, GameStatus, MoveResponse, Coordinate};
use axum::response::IntoResponse;
use rand::{Rng, prelude::ThreadRng};
use rand::seq::SliceRandom; 
use slog::{o, Drain, Logger};
use slog_json::Json as SlogJson;
use std::sync::Mutex;
use serde_json::json;


pub async fn index() -> Json<RootResponse> {
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

pub async fn ping() -> impl IntoResponse {
    println!("got ping");
    (StatusCode::OK, "{}")
}

pub async fn start() -> impl IntoResponse {
    (StatusCode::OK, "{}")
}

fn get_valid_moves(game_status: &GameStatus) -> Vec<&'static str> {
    let possible_moves = ["up", "down", "left", "right"];
    let board_width = game_status.board.width;
    let board_height = game_status.board.height;
    let head = &game_status.you.head;
    let body = &game_status.you.body;

    // Get list of coordinates that other snakes could move into next turn
    let risky_coords: Vec<Coordinate> = game_status.board.snakes.iter()
        .filter(|snake| snake.id != game_status.you.id)
        .flat_map(|snake| {
            vec![
                Coordinate { x: snake.head.x, y: snake.head.y + 1 }, // up
                Coordinate { x: snake.head.x, y: snake.head.y - 1 }, // down
                Coordinate { x: snake.head.x - 1, y: snake.head.y }, // left
                Coordinate { x: snake.head.x + 1, y: snake.head.y }  // right
            ]
        })
        .collect();

    possible_moves.iter().filter_map(|&mv| {
        let (new_x, new_y) = match mv {
            "up" => (head.x, head.y + 1),
            "down" => (head.x, head.y - 1),
            "left" => (head.x - 1, head.y),
            "right" => (head.x + 1, head.y),
            _ => unreachable!(),
        };

        // Create a new Coordinate for the proposed new position
        let new_position = Coordinate { x: new_x, y: new_y };

        if new_x >= 0 
           && new_y >= 0 
           && new_x < board_width 
           && new_y < board_height 
           && !body.contains(&new_position) 
           && !risky_coords.contains(&new_position)
           && game_status.board.snakes.iter().all(|snake| {
               if snake.id == game_status.you.id {
                   true
               } else {
                   !snake.body.contains(&new_position)
               }
           }) {
            Some(mv)
        } else {
            None
        }
    }).collect::<Vec<_>>()
}


pub async fn make_move(game_status: Json<GameStatus>) -> Json<MoveResponse> {
    // Create a root logger
    let drain = SlogJson::default(std::io::stdout()).fuse();
    let drain = Mutex::new(drain).fuse();
    let log = Logger::root(drain, o!());

    // Log the received game state
    let game_status_json = serde_json::to_string(&game_status.0)
        .expect("Failed to serialize game status");
    println!("{}", game_status_json);
    slog::info!(log, "Received game state");

    let valid_moves = get_valid_moves(&game_status.0);
    let mut rng = rand::thread_rng();
    let random_move = valid_moves.choose(&mut rng).unwrap_or(&"up");
    Json(MoveResponse {
        r#move: (*random_move).to_string(),
    })
}

pub async fn end() -> impl IntoResponse {
    StatusCode::OK
}



#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use crate::models::{Coordinate, Snake, Game, Board, GameStatus};

    #[tokio::test]
    async fn test_make_move_doesnt_kill_itself() {
        let game_status = GameStatus {
            game: Game { id: "game_1".to_string() },
            turn: 1,
            board: Board {
                height: 3,
                width: 3,
                food: vec![],
                snakes: vec![],
            },
            you: Snake {
                id: "snake_1".to_string(),
                health: 100,
                body: vec![Coordinate { x: 1, y: 1 }, Coordinate { x: 1, y: 2 }],
                head: Coordinate { x: 1, y: 1 },
                length: 2,
            },
        };

        println!("Board before move:");
        print_board(&game_status);

        let valid_moves = get_valid_moves(&game_status);
        let move_response = make_move(Json(game_status.clone())).await;

        println!("Board after move:");
        println!("Move: {}", move_response.0.r#move);
        assert!(valid_moves.contains(&move_response.0.r#move.as_str()));
    }

    #[test]
    fn test_get_valid_moves_with_long_snake() {
        let game_statuses = vec![
            GameStatus {
                game: Game { id: "game_1".to_string() },
                turn: 1,
                board: Board {
                    height: 3,
                    width: 3,
                    food: vec![],
                    snakes: vec![],
                },
                you: Snake {
                    id: "snake_1".to_string(),
                    health: 100,
                    body: vec![Coordinate { x: 1, y: 1 }, Coordinate { x: 1, y: 2 }, Coordinate { x: 1, y: 0 }],
                    head: Coordinate { x: 1, y: 0 },
                    length: 3,
                },
            },
            GameStatus {
                game: Game { id: "game_2".to_string() },
                turn: 2,
                board: Board {
                    height: 3,
                    width: 3,
                    food: vec![],
                    snakes: vec![],
                },
                you: Snake {
                    id: "snake_2".to_string(),
                    health: 100,
                    body: vec![Coordinate { x: 1, y: 1 }, Coordinate { x: 0, y: 1 }],
                    head: Coordinate { x: 1, y: 1 },
                    length: 2,
                },
            },
            GameStatus {
                game: Game { id: "game_3".to_string() },
                turn: 3,
                board: Board {
                    height: 4,
                    width: 4,
                    food: vec![],
                    snakes: vec![],
                },
                you: Snake {
                    id: "snake_3".to_string(),
                    health: 100,
                    body: vec![Coordinate { x: 1, y: 1 }, Coordinate { x: 1, y: 2 }, Coordinate { x: 1, y: 3 }, Coordinate { x: 2, y: 3 }],
                    head: Coordinate { x: 2, y: 3 },
                    length: 4,
                },
            },
            GameStatus {
                game: Game {
                    id: "df933497-bb5b-4d01-8187-fd50485a8331".to_string(),
                },
                turn: 2,
                you: Snake {
                    id: "gs_XFkjPyBp3w8TSch6FXtXxhxD".to_string(),
                    length: 4,
                    health: 100,
                    head: Coordinate { x: 6, y: 10 },
                    body: vec![
                        Coordinate { x: 6, y: 10 },
                        Coordinate { x: 5, y: 10 },
                        Coordinate { x: 5, y: 9 },
                        Coordinate { x: 5, y: 9 },
                    ],
                },
                board: Board {
                    snakes: vec![
                        Snake {
                            id: "gs_XFkjPyBp3w8TSch6FXtXxhxD".to_string(),
                            length: 4,
                            health: 100,
                            head: Coordinate { x: 6, y: 10 },
                            body: vec![
                                Coordinate { x: 6, y: 10 },
                                Coordinate { x: 5, y: 10 },
                                Coordinate { x: 5, y: 9 },
                                Coordinate { x: 5, y: 9 },
                            ],
                        },
                        Snake {
                            id: "gs_HcBTqW3kwv8x3TFYGCck8gvb".to_string(),
                            length: 4,
                            health: 100,
                            head: Coordinate { x: 4, y: 0 },
                            body: vec![
                                Coordinate { x: 4, y: 0 },
                                Coordinate { x: 4, y: 1 },
                                Coordinate { x: 5, y: 1 },
                                Coordinate { x: 5, y: 1 },
                            ],
                        },
                    ],
                    width: 11,
                    height: 11,
                    food: vec![
                        Coordinate { x: 5, y: 5 },
                        Coordinate { x: 10, y: 3 },
                        Coordinate { x: 1, y: 2 },
                    ],
                },
            }
        ];

        for game_status in game_statuses {
            println!("Board:");
            print_board(&game_status);

            let valid_moves = get_valid_moves(&game_status);

            println!("Valid moves:");
            for move_ in &valid_moves {
                println!("{}", move_);
            }
            println!("\n");
        }
    }

    fn print_board(game_status: &GameStatus) {
        for i in (0..game_status.board.height).rev() {
            for j in 0..game_status.board.width {
                if game_status.you.head == (Coordinate { x: j, y: i }) {
                    print!("H ");
                } else if game_status.you.body.contains(&Coordinate { x: j, y: i }) {
                    print!("B ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
}

