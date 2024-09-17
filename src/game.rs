use crate::snake::*;
use crate::constants::*;

use piston_window::*;
use std::time::{Duration,Instant};

// Enum representing the current status of the game
#[derive(PartialEq)]
pub enum GameStatus {
    Running,
    GameOver,
}

// Enum representing the different types of blocks in the game grid
#[derive(PartialEq,Copy,Clone)]
pub enum BlockType {
    Empty,
    Snake,
    Food,
}

// Struct representing the game state
pub struct Game {
    pub snake: Snake,
    food_exist: bool,
    pub game_status: GameStatus,
    pub grid: [[BlockType;GRID_SIZE];GRID_SIZE],
    pub score: i32,
}

impl Game {
    // Initializes a new game
    pub fn new() -> Game {
        // Create a new game with initial values
        let mut game = Game {
            snake: Snake::new(),
            food_exist: false,
            game_status: GameStatus::Running,
            grid: [[BlockType::Empty;GRID_SIZE];GRID_SIZE],
            score: 0,
        };

        // Initialize the snake at the starting position
        for i in 0..3 {
            let x = i as f64;
            let y = 2.0;
            game.grid[y as usize][x as usize] = BlockType::Snake;
            game.snake.push_front(x, y);
        }
        return game;
    }

    // Handles the player's key press input to change the snake's direction
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

    // Updates the grid based on the snake's movement
    pub fn update_grid(&mut self) {

        // Flag to check if snake ate food
        let mut ate = false;

        // Get the snake's current head position
        if let Some((head_x, head_y)) = self.snake.head() {
            let (new_x, new_y) = match self.snake.dir {
                Direction::Up => (head_x, head_y - 1.0),
                Direction::Down => (head_x, head_y + 1.0),
                Direction::Left => (head_x - 1.0, head_y),
                Direction::Right => (head_x + 1.0, head_y),
            };

            // Collision check
            if new_x < 0.0 || new_x >= GRID_SIZE as f64 || new_y < 0.0 || new_y >= GRID_SIZE as f64 || self.grid[new_y as usize][new_x as usize] == BlockType::Snake {
                self.game_status = GameStatus::GameOver;
                return;
            }

            // Eaten food check
            if self.grid[new_y as usize][new_x as usize] == BlockType::Food {
                ate = true;
                self.score += 1;
                self.food_exist = false;
            }
    
            // Generate food if doesn't exist
            if !self.food_exist {
                self.generate_food();
            }

             // Update the grid with the snake's new head position
            self.grid[new_y as usize][new_x as usize] = BlockType::Snake;
            self.snake.push_front(new_x, new_y);

             // If the snake did not eat, remove the tail segment
            if !ate {
                if let Some(tail_segment) = self.snake.pop_back() {
                    let x = tail_segment.previous_position.0 as usize;
                    let y = tail_segment.previous_position.1 as usize;
                    self.grid[y][x] = BlockType::Empty;
                }
            }
        }

        // Keep the game running
        self.game_status = GameStatus::Running;
    }

    // Cleans up the removed snake segments after a certain duration
    pub fn cleanup(&mut self, update_duration: Duration) {
        let now = Instant::now();

        // Remove segments that have been removed for longer than the update duration
        self.snake.removed_segments.retain(|segment| {
            if let Some(removal_time) = segment.removal_time {
                now.duration_since(removal_time) < update_duration
            } else {
                false // Remove segments without a removal_time
            }
        });

        // Call snake's cleanup function to handle removed segments
        self.snake.cleanup_removed_segments(update_duration);
    }

    // Generates food at a random position in the grid
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
