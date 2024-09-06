extern crate piston_window;
extern crate rand;

mod draw;
mod constants;
mod game;
mod snake;

use constants::*;
use game::*;
use snake::*;
use piston_window::*;


use std::time::{Duration,Instant};

fn main() {
    let assets = "/usr/share/fonts/truetype/ubuntu/";
    let mut window: PistonWindow = 
    WindowSettings::new("Snake", [GRID_SIZE as f64 * BLOCK_SIZE,GRID_SIZE as f64 * BLOCK_SIZE])
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut glyphs = window.load_font(assets.to_string() + "Ubuntu-L.ttf").unwrap();
    let mut last_update = Instant::now();
    let mut new_game = Game::new((GRID_SIZE*64) as f64, (GRID_SIZE*64) as f64);

    while let Some(e) = window.next() {
        if new_game.game_status == GameStatus::GameOver {
            println!("Game Over");
            break;
        }
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            new_game.key_pressed(key);
        }
        
        if last_update.elapsed() >= Duration::from_millis(200) {       
            update_grid(&mut new_game);
            last_update = Instant::now();
        }
        window.draw_2d(&e, |c, g, device | {
            draw::draw(c, g, &mut new_game, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn update_grid(game: &mut Game) {
    let mut ate = false;
    if let Some(head) = game.snake.head() {
        match game.snake.dir {
            Direction::Up => {
                if head.0 > 0 && game.grid[head.0-1][head.1]!=BlockType::Snake {
                    if game.grid[head.0-1][head.1] == BlockType::Food {
                        ate = true;
                    }
                    game.grid[head.0-1][head.1] = BlockType::Snake;
                    game.snake.push_front(head.0-1,head.1);

                } else {
                    game.game_status = GameStatus::GameOver;
                    return;
                }
            }
            Direction::Down => {
                if head.0 < GRID_SIZE-1 && game.grid[head.0+1][head.1]!=BlockType::Snake {
                    if game.grid[head.0+1][head.1] == BlockType::Food {
                        ate = true;
                    }
                    game.snake.push_front(head.0+1,head.1);
                    game.grid[head.0+1][head.1] = BlockType::Snake;
                } else {
                    game.game_status = GameStatus::GameOver;
                    return;
                }
            }
            Direction::Left => {
                if head.1 > 0 && game.grid[head.0][head.1-1]!=BlockType::Snake {    
                    if game.grid[head.0][head.1-1] == BlockType::Food {
                        ate = true;
                    }
                    game.snake.push_front(head.0,head.1-1);
                    game.grid[head.0][head.1-1] = BlockType::Snake;
                } else {
                    game.game_status = GameStatus::GameOver;
                    return;
                }
            }
            Direction::Right => {
                if head.1 < GRID_SIZE-1 && game.grid[head.0][head.1+1]!=BlockType::Snake {
                    if game.grid[head.0][head.1+1] == BlockType::Food {
                        ate = true;
                    }
                    game.snake.push_front(head.0,head.1+1);
                    game.grid[head.0][head.1+1] = BlockType::Snake;
                } else {
                    game.game_status = GameStatus::GameOver;
                    return;
                }
            }
        }
        if !game.food_exist {
            let mut finished = false;
            while !finished {
                let x: usize = (rand::random::<f64>()*100.0) as usize % GRID_SIZE;
                let y: usize = (rand::random::<f64>()*100.0) as usize % GRID_SIZE;
                if game.grid[x][y] == BlockType::Empty {
                    finished = true;
                    game.grid[x][y] = BlockType::Food;
                }
            }
            game.food_exist = true;
        }
        if !ate {
            if let Some(tail) = game.snake.pop_back() {
                game.grid[tail.0][tail.1] = BlockType::Empty;
            }
        } else {
            game.score += 1;
            game.food_exist = false;
        }
    }
    game.game_status = GameStatus::Running;
}
