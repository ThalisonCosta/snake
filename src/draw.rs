use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

fn to_coord(coord: i32) -> f64 {
    f64::from(coord) * BLOCK_SIZE
}

pub fn to_coord_u32(coord: i32) -> u32 {
    to_coord(coord) as u32
}

pub fn draw_block(color: Color, x:i32, y: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(color, [x, y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_rectangle(color: Color, x:i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(color, [x, y, BLOCK_SIZE * (f64::from(width)), BLOCK_SIZE * (f64::from(height))], con.transform, g);
}
