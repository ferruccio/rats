use video::Size;

mod baby_rat;
mod bullet;
mod player;
mod position;
mod rat;
mod rat_factory;

pub use baby_rat::*;
pub use bullet::*;
pub use player::*;
pub use position::*;
pub use rat::*;
pub use rat_factory::*;

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    _Rat(Rat),
    _BabyRat(BabyRat),
    _RatFactory(RatFactory),
    Bullet(Bullet),
}

pub type EntityList = Vec<Entity>;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub rows: Size,
    pub cols: Size,
}

pub type Direction = u8;
pub mod dir {
    use super::Direction;
    pub const NONE: Direction = 0x00;
    pub const UP: Direction = 0x01;
    pub const DOWN: Direction = 0x02;
    pub const LEFT: Direction = 0x04;
    pub const RIGHT: Direction = 0x08;
    pub const UP_LEFT: Direction = UP | LEFT;
    pub const UP_RIGHT: Direction = UP | RIGHT;
    pub const DOWN_LEFT: Direction = DOWN | LEFT;
    pub const DOWN_RIGHT: Direction = DOWN | RIGHT;

    pub fn stop_dir(dir: Direction) -> Direction {
        match dir {
            UP | DOWN | LEFT | RIGHT => dir,
            UP_LEFT | DOWN_LEFT => LEFT,
            UP_RIGHT | DOWN_RIGHT => RIGHT,
            _ => UP,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Alive,
    _Exploding,
    _Dead,
}
