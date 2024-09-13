extern crate piston_window;
extern crate rand;

mod draw;
mod constants;
mod game;
mod snake;

use constants::*;
use game::*;
use piston_window::*;


use std::time::{Duration,Instant};

fn main() {
    let assets = "assets/Ubuntu-M.ttf";
    let mut window: PistonWindow = 
    WindowSettings::new("Snake", [GRID_SIZE as f64 * BLOCK_SIZE,GRID_SIZE as f64 * BLOCK_SIZE])
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut glyphs = window.load_font(assets.to_string()).unwrap();
    let mut last_update = Instant::now();
    let update_duration = Duration::from_millis(200);
    let mut new_game = Game::new();

    while let Some(e) = window.next() {
        if new_game.game_status == GameStatus::GameOver {
            println!("Game Over");
            println!("Score: {}",new_game.score);
            break;
        }
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            new_game.key_pressed(key);
        }

        let now = Instant::now();
        let elapsed = now.duration_since(last_update);
        
        if elapsed >= update_duration {       
            new_game.update_grid();
            new_game.cleanup(update_duration);
            last_update = now;
        }

        let mut interpolation = elapsed.as_secs_f64() / update_duration.as_secs_f64();
        if interpolation > 1.0 {
            interpolation = 1.0;
        }

        window.draw_2d(&e, |c, g, device | {
            draw::draw(c, g, &mut new_game, &mut glyphs, interpolation);
            glyphs.factory.encoder.flush(device);
        });
    }
}