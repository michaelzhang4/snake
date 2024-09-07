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
    snake: Snake,
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
            game.grid[0][i] = BlockType::Snake;
            game.snake.push_front(0,i);
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
        if let Some(head) = self.snake.head() {
            match self.snake.dir {
                Direction::Up => {
                    if head.0 > 0 && self.grid[head.0-1][head.1]!=BlockType::Snake {
                        if self.grid[head.0-1][head.1] == BlockType::Food {
                            ate = true;
                        }
                        self.grid[head.0-1][head.1] = BlockType::Snake;
                        self.snake.push_front(head.0-1,head.1);
    
                    } else {
                        self.game_status = GameStatus::GameOver;
                        return;
                    }
                }
                Direction::Down => {
                    if head.0 < GRID_SIZE-1 && self.grid[head.0+1][head.1]!=BlockType::Snake {
                        if self.grid[head.0+1][head.1] == BlockType::Food {
                            ate = true;
                        }
                        self.snake.push_front(head.0+1,head.1);
                        self.grid[head.0+1][head.1] = BlockType::Snake;
                    } else {
                        self.game_status = GameStatus::GameOver;
                        return;
                    }
                }
                Direction::Left => {
                    if head.1 > 0 && self.grid[head.0][head.1-1]!=BlockType::Snake {    
                        if self.grid[head.0][head.1-1] == BlockType::Food {
                            ate = true;
                        }
                        self.snake.push_front(head.0,head.1-1);
                        self.grid[head.0][head.1-1] = BlockType::Snake;
                    } else {
                        self.game_status = GameStatus::GameOver;
                        return;
                    }
                }
                Direction::Right => {
                    if head.1 < GRID_SIZE-1 && self.grid[head.0][head.1+1]!=BlockType::Snake {
                        if self.grid[head.0][head.1+1] == BlockType::Food {
                            ate = true;
                        }
                        self.snake.push_front(head.0,head.1+1);
                        self.grid[head.0][head.1+1] = BlockType::Snake;
                    } else {
                        self.game_status = GameStatus::GameOver;
                        return;
                    }
                }
            }
            if !self.food_exist {
                let mut finished = false;
                while !finished {
                    let x: usize = (rand::random::<f64>()*100.0) as usize % GRID_SIZE;
                    let y: usize = (rand::random::<f64>()*100.0) as usize % GRID_SIZE;
                    if self.grid[x][y] == BlockType::Empty {
                        finished = true;
                        self.grid[x][y] = BlockType::Food;
                    }
                }
                self.food_exist = true;
            }
            if !ate {
                if let Some(tail) = self.snake.pop_back() {
                    self.grid[tail.0][tail.1] = BlockType::Empty;
                }
            } else {
                self.score += 1;
                self.food_exist = false;
            }
        }
        self.game_status = GameStatus::Running;
    }
}
