use crate::CHAR_CELL_HEIGHT;

pub const BULLETS_START: u8 = 0x90;
pub const BULLET_UP: u8 = BULLETS_START;
pub const BULLET_DOWN: u8 = BULLETS_START + 1;
pub const BULLET_LEFT: u8 = BULLETS_START + 2;
pub const BULLET_RIGHT: u8 = BULLETS_START + 3;
pub const BULLET_UP_LEFT: u8 = BULLETS_START + 4;
pub const BULLET_UP_RIGHT: u8 = BULLETS_START + 5;
pub const BULLET_DOWN_LEFT: u8 = BULLETS_START + 6;
pub const BULLET_DOWN_RIGHT: u8 = BULLETS_START + 7;

// comments on each line prevent rustfmt from changing this layout
pub const BULLETS: [u8; 8 * CHAR_CELL_HEIGHT] = [
    // up
    // 01234567
    0b_00000000, // 00
    0b_00000000, // 01
    0b_00010000, // 02
    0b_00111000, // 03
    0b_01111100, // 04
    0b_01111100, // 05
    0b_01111100, // 06
    0b_01111100, // 07
    0b_00000000, // 08
    0b_01000100, // 09
    0b_00010000, // 10
    0b_00000000, // 11
    // down
    // 01234567
    0b_00000000, // 00
    0b_00010000, // 01
    0b_01000100, // 02
    0b_00000000, // 03
    0b_01111100, // 05
    0b_01111100, // 05
    0b_01111100, // 06
    0b_01111100, // 07
    0b_00111000, // 08
    0b_00010000, // 09
    0b_00000000, // 10
    0b_00000000, // 11
    // left
    // 01234567
    0b_00000000, // 00
    0b_00000000, // 01
    0b_00000000, // 02
    0b_00000000, // 03
    0b_00111101, // 04
    0b_01111100, // 05
    0b_11111101, // 06
    0b_01111100, // 07
    0b_00111101, // 08
    0b_00000000, // 09
    0b_00000000, // 10
    0b_00000000, // 11
    // right
    // 01234567
    0b_00000000, // 00
    0b_00000000, // 01
    0b_00000000, // 02
    0b_00000000, // 03
    0b_10111100, // 04
    0b_00111110, // 05
    0b_10111111, // 06
    0b_00111110, // 07
    0b_10111100, // 08
    0b_00000000, // 09
    0b_00000000, // 10
    0b_00000000, // 11
    // up left
    // 01234567
    0b_00000000, // 00
    0b_00000000, // 01
    0b_11110000, // 02
    0b_11111000, // 03
    0b_11111100, // 04
    0b_11111001, // 05
    0b_01110000, // 06
    0b_00100010, // 07
    0b_00001000, // 08
    0b_00000000, // 09
    0b_00000000, // 10
    0b_00000000, // 11
    // up right
    // 01234567
    0b_00000000, // 00
    0b_00000000, // 01
    0b_00001111, // 02
    0b_00011111, // 03
    0b_00111111, // 04
    0b_10011111, // 05
    0b_00001110, // 06
    0b_01000100, // 07
    0b_00010000, // 08
    0b_00000000, // 09
    0b_00000000, // 10
    0b_00000000, // 11
    // down left
    // 01234567
    0b_00000000, // 00
    0b_00000000, // 01
    0b_00000000, // 02
    0b_00001000, // 03
    0b_00100010, // 04
    0b_01110000, // 05
    0b_11111001, // 06
    0b_11111100, // 07
    0b_11111000, // 08
    0b_11110000, // 09
    0b_00000000, // 10
    0b_00000000, // 11
    // down right
    // 01234567
    0b_00000000, // 00
    0b_00000000, // 01
    0b_00000000, // 02
    0b_00010000, // 03
    0b_01000100, // 04
    0b_00001110, // 05
    0b_10011111, // 06
    0b_00111111, // 07
    0b_00011111, // 08
    0b_00001111, // 09
    0b_00000000, // 10
    0b_00000000, // 11
];
