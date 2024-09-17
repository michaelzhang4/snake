extern crate piston_window;
extern crate rand;

// Import modules
mod draw;
mod constants;
mod game;
mod snake;

use constants::*;
use game::*;
use piston_window::*;

use std::time::{Duration,Instant};

fn main() {
    // Create a new PistonWindow instance with title "Snake" and set its size
    let mut window: PistonWindow = 
    WindowSettings::new("Snake", [GRID_SIZE as f64 * BLOCK_SIZE,GRID_SIZE as f64 * BLOCK_SIZE])
    .exit_on_esc(true) // Allow exiting the game using the Escape key
    .samples(4) // Anti-aliasing (makes edges smooth)
    .build()
    .unwrap();  // Unwrap to panic if window creation fails
    
    let mut last_update = Instant::now(); // Track the last time the game was updated
    let update_duration = Duration::from_millis(200); // Time between game updates (200ms)
    let mut lag = Duration::from_secs(0); // Lag between frame updates

    let mut new_game = Game::new(); // Initialise the game
    
    // Load font for rendering text (score, etc.)
    let assets = "assets/Roboto-Black.ttf";
    let texture_settings = TextureSettings::new().filter(Filter::Linear);
    let mut glyphs = Glyphs::new(
        assets,
        window.create_texture_context(),
        texture_settings,
    ).expect("Could not load font");

    // Main game loop
    while let Some(e) = window.next() {

        // Check if the game is over
        if new_game.game_status == GameStatus::GameOver {
            println!("Game Over");
            println!("Score: {}",new_game.score);
            break;
        }
        
        // Handle player input: Check if any keyboard key was pressed
        if let Some(Button::Keyboard(key)) = e.press_args() {
            new_game.key_pressed(key);
        }

        // Calculate the time elapsed since the last update
        let current = Instant::now();
        let elapsed = current.duration_since(last_update);
        last_update = current;
        lag += elapsed;
        
        // Update the game state if enough time has passed (according to update_duration)
        if lag >= update_duration {       
            new_game.update_grid();
            new_game.cleanup(update_duration);
            lag -= update_duration;
        }

        // Calculate the interpolation value to ensure smooth movement
        let interpolation = lag.as_secs_f64() / update_duration.as_secs_f64();

        // Draw the game
        window.draw_2d(&e, |c, g, device | {
            draw::draw(c, g, &mut new_game, &mut glyphs, interpolation);
            glyphs.factory.encoder.flush(device);
        });
    }
}