use crate::CHAR_CELL_HEIGHT;

mod ascii;
mod booms;
mod brats;
mod bullets;
mod factories;
mod maze_walls;
mod player;
mod rats;

pub use ascii::*;
pub use booms::*;
pub use brats::*;
pub use bullets::*;
pub use factories::*;
pub use maze_walls::*;
pub use player::*;
pub use rats::*;

// comments on each line prevent rustfmt from changing this layout
pub const EMPTY_CHAR_CELL: [u8; CHAR_CELL_HEIGHT] = [
    0b_00000000, // 00
    0b_00000000, // 01
    0b_00000000, // 02
    0b_00000000, // 03
    0b_00111100, // 04
    0b_00100100, // 05
    0b_00100100, // 06
    0b_00111100, // 07
    0b_00000000, // 08
    0b_00000000, // 09
    0b_00000000, // 10
    0b_00000000, // 11
];

pub const BIG_BLANK_START: u8 = 0x18;
pub const BIG_BLANK: [u16; CHAR_CELL_HEIGHT * 2] = [0; CHAR_CELL_HEIGHT * 2];

pub const WHITE: u32 = 0xffffff;
pub const RED: u32 = 0xff0000;
pub const GREEN: u32 = 0x00ff00;
pub const BLUE: u32 = 0x0000ff;
pub const YELLOW: u32 = RED | GREEN;
pub const CYAN: u32 = GREEN | BLUE;
pub const CRT_GREEN: u32 = 0x80ff80;
pub const CRT_BACKGROUND: u32 = 0x252919;
