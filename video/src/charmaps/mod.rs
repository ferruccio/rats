use crate::CHAR_CELL_HEIGHT;

mod ascii;
mod bullets;
mod factories;
mod maze_walls;
mod player;
mod rats;

pub use ascii::*;
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

pub const BRATS_START: u8 = b'a';
pub const BRATS_UP_A1: u8 = BRATS_START;
pub const BRATS_UP_A2: u8 = BRATS_START + 1;
pub const BRATS_DOWN_A1: u8 = BRATS_START + 2;
pub const BRATS_DOWN_A2: u8 = BRATS_START + 3;
pub const BRATS_LEFT_A1: u8 = BRATS_START + 4;
pub const BRATS_LEFT_A2: u8 = BRATS_START + 5;
pub const BRATS_RIGHT_A1: u8 = BRATS_START + 6;
pub const BRATS_RIGHT_A2: u8 = BRATS_START + 7;

pub const BIG_BOOM_START: u8 = b'A';
pub const BIG_BOOM_A1: u8 = BIG_BOOM_START;
pub const BIG_BOOM_A2: u8 = BIG_BOOM_START + 4;

pub const LIL_BOOM_START: u8 = b'a';
pub const LIL_BOOM_A1: u8 = LIL_BOOM_START;
pub const LIL_BOOM_A2: u8 = LIL_BOOM_START + 1;
