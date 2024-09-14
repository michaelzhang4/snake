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
    let mut window: PistonWindow = 
    WindowSettings::new("Snake", [GRID_SIZE as f64 * BLOCK_SIZE,GRID_SIZE as f64 * BLOCK_SIZE])
    .exit_on_esc(true)
    .samples(4)
    .build()
    .unwrap();
    
    let mut last_update = Instant::now();
    let update_duration = Duration::from_millis(200);
    let mut lag = Duration::from_secs(0);
    let mut new_game = Game::new();
    
    let assets = "assets/Roboto-Black.ttf";
    let texture_settings = TextureSettings::new().filter(Filter::Linear);
    let mut glyphs = Glyphs::new(
        assets,
        window.create_texture_context(),
        texture_settings,
    ).expect("Could not load font");

    while let Some(e) = window.next() {
        if new_game.game_status == GameStatus::GameOver {
            println!("Game Over");
            println!("Score: {}",new_game.score);
            break;
        }
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            new_game.key_pressed(key);
        }

        let current = Instant::now();
        let elapsed = current.duration_since(last_update);
        last_update = current;
        lag += elapsed;
        
        if lag >= update_duration {       
            new_game.update_grid();
            new_game.cleanup(update_duration);
            lag -= update_duration;
        }

        let interpolation = lag.as_secs_f64() / update_duration.as_secs_f64();

        window.draw_2d(&e, |c, g, device | {
            draw::draw(c, g, &mut new_game, &mut glyphs, interpolation);
            glyphs.factory.encoder.flush(device);
        });
    }
}