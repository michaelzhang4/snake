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
    let mut new_game = Game::new();

    while let Some(e) = window.next() {
        if new_game.game_status == GameStatus::GameOver {
            println!("Game Over");
            break;
        }
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            new_game.key_pressed(key);
        }
        
        if last_update.elapsed() >= Duration::from_millis(200) {       
            new_game.update_grid();
            last_update = Instant::now();
        }
        window.draw_2d(&e, |c, g, device | {
            draw::draw(c, g, &mut new_game, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });
    }
}