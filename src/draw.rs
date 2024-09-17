use crate::constants::*;
use crate::game::BlockType;
use crate::game::Game;

use piston_window::*;

// Function to render the current state of the game onto the screen
pub fn draw(c: Context, g: &mut G2d, game: &mut Game, glyphs: &mut Glyphs, interpolation: f64) {

    // Clear the screen with a white background color
    clear(color::WHITE, g);

    // Iterate over each cell in the grid
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            match game.grid[y][x] {
                BlockType::Empty => {
                    // Skip
                    //rectangle(color::WHITE, [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
                BlockType::Snake => {
                    // Skip
                    //rectangle(color::GREEN, [BLOCK_SIZE*j as f64, BLOCK_SIZE*i as f64, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
                BlockType::Food => {
                    rectangle(color::RED, [BLOCK_SIZE*x as f64, BLOCK_SIZE*y as f64, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
            }
        }
    }

     // Draw each segment of the snake's body with interpolation for smooth movement
    for segment in &game.snake.body {

        // Calculate the interpolated position based on previous and current positions
        let interpolated_x = segment.previous_position.0 + (segment.current_position.0 - segment.previous_position.0) * interpolation;
        let interpolated_y = segment.previous_position.1 + (segment.current_position.1 - segment.previous_position.1) * interpolation;

         // Check if the segment is moving horizontally or vertically and render block accordingly
        if segment.current_position.0-segment.previous_position.0 == 0.0 {
            rectangle(color::GREEN,[interpolated_x * BLOCK_SIZE + DIFF, interpolated_y * BLOCK_SIZE, SNAKE_SIZE, BLOCK_SIZE - DIFF], c.transform, g);
        }
        else if segment.current_position.1-segment.previous_position.1 == 0.0 {
            rectangle(color::GREEN,[interpolated_x * BLOCK_SIZE, interpolated_y * BLOCK_SIZE + DIFF, BLOCK_SIZE - DIFF, SNAKE_SIZE], c.transform, g);
        }
    }

     // Draw any segments that have been removed (e.g., when the snake moves forward)
    for segment in &game.snake.removed_segments {

        // Calculate the interpolated position for smooth fading
        let interpolated_x = segment.previous_position.0 + (segment.current_position.0 - segment.previous_position.0) * interpolation;
        let interpolated_y = segment.previous_position.1 + (segment.current_position.1 - segment.previous_position.1) * interpolation;

        // Draw the removed segment as a green rectangle
        rectangle(color::GREEN,[interpolated_x * BLOCK_SIZE, interpolated_y * BLOCK_SIZE, SNAKE_SIZE, SNAKE_SIZE], c.transform, g);
    }

    // Convert the player's score to a string for display
    let num_str = game.score.to_string();

    // Render the score text at the top center of the screen
    let _ = text(color::BLACK, 60, &num_str, glyphs, c.transform.trans((GRID_SIZE as f64-1.0)*64.0 / 2.0,70.0), g);
}