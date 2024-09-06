use crate::constants::GRID_SIZE;
use crate::constants::BLOCK_SIZE;
use crate::game::BlockType;
use crate::game::Game;

use piston_window::*;


pub fn draw(c: Context, g: &mut G2d, game: &mut Game, glyphs: &mut Glyphs ) {
    let num_str = game.score.to_string();
    clear(color::WHITE, g);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            match game.grid[i][j] {
                BlockType::Empty => {
                    //rectangle(color::WHITE, [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
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
    let _ = text(color::BLACK, 48, &num_str, glyphs, c.transform.trans(GRID_SIZE as f64*64.0 / 2.0,50.0), g);
}