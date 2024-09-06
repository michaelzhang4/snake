use crate::snake::*;
use crate::constants::*;
use piston_window::*;

#[derive(PartialEq)]
pub enum GameStatus {
    Running,
    GameOver,
}

#[derive(PartialEq,Copy,Clone)]
pub enum BlockType {
    Empty,
    Snake,
    Food,
}

pub struct Game {
    pub snake: Snake,
    pub food_exist: bool,
    pub game_status: GameStatus,
    pub speed: f64,
    height: f64,
    width: f64,
    pub grid: [[BlockType;GRID_SIZE];GRID_SIZE],
    pub score: i32,
}

impl Game {
    pub fn new(width: f64, height: f64) -> Game {
        let mut game = Game {
            snake: Snake::new(),
            food_exist: false,
            game_status: GameStatus::Running,
            speed: 0.2,
            height,
            width,
            grid: [[BlockType::Empty;GRID_SIZE];GRID_SIZE],
            score: 0,
        };

        for i in 0..3 {
            game.grid[0][i] = BlockType::Snake;
            game.snake.push_front(0,i);
        }
        return game;
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::W => {
                if self.snake.dir!=Direction::Down {
                    self.snake.dir = Direction::Up;
                }                    
            }
            Key::S => {
                if self.snake.dir!=Direction::Up {
                    self.snake.dir = Direction::Down;
                }
            }
            Key::A => {
                if self.snake.dir!=Direction::Right {
                    self.snake.dir = Direction::Left;
                }
            }
            Key::D => {
                if self.snake.dir!=Direction::Left {
                    self.snake.dir = Direction::Right;
                }
            }
            _ => {
                ();
            }
        }
    }
}