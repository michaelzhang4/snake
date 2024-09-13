use crate::constants::*;
use crate::game::BlockType;
use crate::game::Game;

use piston_window::*;


pub fn draw(c: Context, g: &mut G2d, game: &mut Game, glyphs: &mut Glyphs, interpolation: f64) {
    clear(color::WHITE, g);
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            match game.grid[y][x] {
                BlockType::Empty => {
                    //rectangle(color::WHITE, [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
                BlockType::Snake => {
                    //rectangle(color::GREEN, [BLOCK_SIZE*j as f64, BLOCK_SIZE*i as f64, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
                BlockType::Food => {
                    rectangle(color::RED, [BLOCK_SIZE*x as f64, BLOCK_SIZE*y as f64, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
                }
            }
        }
    }
    for segment in &game.snake.body {
        
        let interpolated_x = segment.previous_position.0 + (segment.current_position.0 - segment.previous_position.0) * interpolation;
        let interpolated_y = segment.previous_position.1 + (segment.current_position.1 - segment.previous_position.1) * interpolation;
        if segment.current_position.0-segment.previous_position.0 == 0.0 {
            rectangle(color::GREEN,[interpolated_x * BLOCK_SIZE + DIFF, interpolated_y * BLOCK_SIZE, SNAKE_SIZE, BLOCK_SIZE], c.transform, g);
        }
        if segment.current_position.1-segment.previous_position.1 == 0.0 {
            rectangle(color::GREEN,[interpolated_x * BLOCK_SIZE, interpolated_y * BLOCK_SIZE + DIFF, BLOCK_SIZE, SNAKE_SIZE], c.transform, g);
        }
    }

    for segment in &game.snake.removed_segments {
        let interpolated_x = segment.previous_position.0 + (segment.current_position.0 - segment.previous_position.0) * interpolation;
        let interpolated_y = segment.previous_position.1 + (segment.current_position.1 - segment.previous_position.1) * interpolation;

        rectangle(color::GREEN,[interpolated_x * BLOCK_SIZE, interpolated_y * BLOCK_SIZE, SNAKE_SIZE, SNAKE_SIZE], c.transform, g);
    }

    let num_str = game.score.to_string();
    let _ = text(color::BLACK, 48, &num_str, glyphs, c.transform.trans(GRID_SIZE as f64*64.0 / 2.0,50.0), g);
}