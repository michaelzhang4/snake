extern crate piston_window;
extern crate rand;

use glyph_cache::rusttype::GlyphCache;
use piston_window::*;
use piston_window::{Glyphs, text};


use std::collections::VecDeque;
use std::time::{Duration,Instant};

const BLOCK_SIZE: f64 = 64.0;
const GRID_SIZE: usize = 9;

#[derive(PartialEq,Copy,Clone)]
enum BlockType {
    Empty,
    Snake,
    Food,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum GameStatus {
    Running,
    GameOver,
}

struct Snake {
    dir: Direction,
    body: VecDeque<(usize,usize)>
}

fn main() {
    let assets = "/usr/share/fonts/truetype/ubuntu/";
    let mut score = 0;
    let mut window: PistonWindow = 
    WindowSettings::new("Snake", [GRID_SIZE as f64 * BLOCK_SIZE,GRID_SIZE as f64 * BLOCK_SIZE])
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut glyphs = window.load_font(assets.to_string() + "Ubuntu-L.ttf").unwrap();
    let mut grid: [[BlockType;GRID_SIZE];GRID_SIZE] = [[BlockType::Empty;GRID_SIZE];GRID_SIZE];
    let mut snake = Snake{dir: Direction::Right, body: VecDeque::new()};

    for i in 0..3 {
        grid[0][i] = BlockType::Snake;
        snake.body.push_front((0,i));
    }

    let mut game_status = GameStatus::Running;

    let mut last_update = Instant::now();

    let mut fruit  = false;

    while let Some(e) = window.next() {
        if game_status == GameStatus::GameOver {
            println!("Game Over");
            break;
        }
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => {
                    if snake.dir!=Direction::Down {
                        snake.dir = Direction::Up;
                    }                    
                }
                Key::S => {
                    if snake.dir!=Direction::Up {
                        snake.dir = Direction::Down;
                    }
                }
                Key::A => {
                    if snake.dir!=Direction::Right {
                        snake.dir = Direction::Left;
                    }
                }
                Key::D => {
                    if snake.dir!=Direction::Left {
                        snake.dir = Direction::Right;
                    }
                }
                _ => {
                    ();
                }
            }
        }
        
        if last_update.elapsed() >= Duration::from_millis(200) {       
            game_status = update_grid(&mut snake, &mut grid, &mut fruit, &mut score);
            last_update = Instant::now();
        }
        window.draw_2d(&e, |c, g, device | {
            draw(c, g, &mut grid, &score, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn draw(c: Context, g: &mut G2d, grid: &mut[[BlockType;GRID_SIZE];GRID_SIZE], score: &i32, glyphs: &mut Glyphs ) {
    let num_str = score.to_string();
    clear(color::WHITE, g);
    let _ = text(color::BLACK, 48, &num_str, glyphs, c.transform.trans(GRID_SIZE as f64*64.0 / 2.0,50.0), g);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            match grid[i][j] {
                BlockType::Empty => {
                    //rectangle(color::GREEN, [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
                BlockType::Snake => {
                    rectangle(color::GREEN, [BLOCK_SIZE*j as f64, BLOCK_SIZE*i as f64, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
                BlockType::Food => {
                    rectangle(color::RED, [BLOCK_SIZE*j as f64, BLOCK_SIZE*i as f64, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
            }
        }
    }
}

fn update_grid(snake: &mut Snake, grid: &mut[[BlockType;GRID_SIZE];GRID_SIZE], fruit: &mut bool, score: &mut i32) -> GameStatus {
    let mut ate = false;
    if let Some(&head) = snake.body.front() {
        match snake.dir {
            Direction::Up => {
                if head.0 > 0 && grid[head.0-1][head.1]!=BlockType::Snake {
                    if grid[head.0-1][head.1] == BlockType::Food {
                        ate = true;
                    }
                    grid[head.0-1][head.1] = BlockType::Snake;
                    snake.body.push_front((head.0-1,head.1));

                } else {
                    return GameStatus::GameOver;
                }
            }
            Direction::Down => {
                if head.0 < GRID_SIZE-1 && grid[head.0+1][head.1]!=BlockType::Snake {
                    if grid[head.0+1][head.1] == BlockType::Food {
                        ate = true;
                    }
                    snake.body.push_front((head.0+1,head.1));
                    grid[head.0+1][head.1] = BlockType::Snake;
                } else {
                    return GameStatus::GameOver;
                }
            }
            Direction::Left => {
                if head.1 > 0 && grid[head.0][head.1-1]!=BlockType::Snake {    
                    if grid[head.0][head.1-1] == BlockType::Food {
                        ate = true;
                    }
                    snake.body.push_front((head.0,head.1-1));
                    grid[head.0][head.1-1] = BlockType::Snake;
                } else {
                    return GameStatus::GameOver;
                }
            }
            Direction::Right => {
                if head.1 < GRID_SIZE-1 && grid[head.0][head.1+1]!=BlockType::Snake {
                    if grid[head.0][head.1+1] == BlockType::Food {
                        ate = true;
                    }
                    snake.body.push_front((head.0,head.1+1));
                    grid[head.0][head.1+1] = BlockType::Snake;
                } else {
                    return GameStatus::GameOver;
                }
            }
        }
        if !*fruit {
            let mut finished = false;
            while !finished {
                let x: usize = (rand::random::<f64>()*100.0) as usize % GRID_SIZE;
                let y: usize = (rand::random::<f64>()*100.0) as usize % GRID_SIZE;
                if grid[x][y] == BlockType::Empty {
                    finished = true;
                    grid[x][y] = BlockType::Food;
                }
            }
            *fruit = true;
        }
        if !ate {
            if let Some(tail) = snake.body.pop_back() {
                grid[tail.0][tail.1] = BlockType::Empty;
            }
        } else {
            *score += 1;
            *fruit = false;
        }
    }
    GameStatus::Running
}
