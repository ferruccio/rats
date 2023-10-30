use crate::CHAR_CELL_HEIGHT;

mod ascii;
mod maze_walls;
mod player;

pub use ascii::*;
pub use maze_walls::*;
pub use player::*;

// comments on each line prevent rustfmt from changing this layout
pub const EMPTY_CHAR_CELL: [u8; CHAR_CELL_HEIGHT] = [
    0b_00000000, //
    0b_00000000, //
    0b_00000000, //
    0b_00000000, //
    0b_00111100, //
    0b_00100100, //
    0b_00100100, //
    0b_00111100, //
    0b_00000000, //
    0b_00000000, //
    0b_00000000, //
    0b_00000000, //
];
