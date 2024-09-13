use crate::snake::*;
use crate::constants::*;

use piston_window::*;
use std::time::{Duration,Instant};

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
    food_exist: bool,
    pub game_status: GameStatus,
    pub grid: [[BlockType;GRID_SIZE];GRID_SIZE],
    pub score: i32,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            snake: Snake::new(),
            food_exist: false,
            game_status: GameStatus::Running,
            grid: [[BlockType::Empty;GRID_SIZE];GRID_SIZE],
            score: 0,
        };

        for i in 0..3 {
            let x = i as f64;
            let y = 2.0;
            game.grid[y as usize][x as usize] = BlockType::Snake;
            game.snake.push_front(x, y);
        }
        return game;
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::W | Key::Up => {
                if self.snake.dir!=Direction::Down {
                    self.snake.dir = Direction::Up;
                }                    
            }
            Key::S | Key::Down => {
                if self.snake.dir!=Direction::Up {
                    self.snake.dir = Direction::Down;
                }
            }
            Key::A | Key::Left => {
                if self.snake.dir!=Direction::Right {
                    self.snake.dir = Direction::Left;
                }
            }
            Key::D | Key::Right => {
                if self.snake.dir!=Direction::Left {
                    self.snake.dir = Direction::Right;
                }
            }
            _ => {
                ();
            }
        }
    }

    pub fn update_grid(&mut self) {

        let mut ate = false;
        // Directions
        if let Some((head_x, head_y)) = self.snake.head() {
            let (new_x, new_y) = match self.snake.dir {
                Direction::Up => (head_x, head_y - 1.0),
                Direction::Down => (head_x, head_y + 1.0),
                Direction::Left => (head_x - 1.0, head_y),
                Direction::Right => (head_x + 1.0, head_y),
            };

            // Boundary check
            if new_x < 0.0 || new_x >= GRID_SIZE as f64 || new_y < 0.0 || new_y >= GRID_SIZE as f64 || self.grid[new_y as usize][new_x as usize] == BlockType::Snake {
                self.game_status = GameStatus::GameOver;
                return;
            }

            // Food check
            if self.grid[new_y as usize][new_x as usize] == BlockType::Food {
                ate = true;
                self.score += 1;
                self.food_exist = false;
            }
    
            if !self.food_exist {
                self.generate_food();
            }

            self.grid[new_y as usize][new_x as usize] = BlockType::Snake;
            self.snake.push_front(new_x, new_y);

            if !ate {
                if let Some(tail_segment) = self.snake.pop_back() {
                    let x = tail_segment.previous_position.0 as usize;
                    let y = tail_segment.previous_position.1 as usize;
                    self.grid[y][x] = BlockType::Empty;
                }
            }
        }
        self.game_status = GameStatus::Running;
    }

    pub fn cleanup(&mut self, update_duration: Duration) {
        let now = Instant::now();
        self.snake.removed_segments.retain(|segment| {
            if let Some(removal_time) = segment.removal_time {
                now.duration_since(removal_time) < update_duration
            } else {
                false // Remove segments without a removal_time
            }
        });
        self.snake.cleanup_removed_segments(update_duration);
    }

    fn generate_food(&mut self) {
        loop {
            let x: usize = rand::random::<usize>() % GRID_SIZE;
            let y: usize = rand::random::<usize>() % GRID_SIZE;
            if self.grid[y][x] == BlockType::Empty {
                self.grid[y][x] = BlockType::Food;
                self.food_exist = true;
                break;
            }
        }
    }
}
