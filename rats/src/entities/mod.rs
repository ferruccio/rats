use video::Size;

mod brat;
mod bullet;
mod factory;
mod player;
mod position;
mod rat;

pub use brat::*;
pub use bullet::*;
pub use factory::*;
pub use player::*;
pub use position::*;
pub use rat::*;

pub trait EntityAction {
    fn hit(&self, pos: Position, dims: Dimensions) -> bool;
    fn explode(&mut self);
}

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    Rat(Rat),
    Brat(Brat),
    Factory(Factory),
    Bullet(Bullet),
}

// milliseconds between updates
pub const PLAYER_UPDATE_MS: u32 = 100;
pub const RAT_UPDATE_MS: u32 = 100;
pub const BRAT_UPDATE_MS: u32 = 100;
pub const FACTORY_UPDATE_MS: u32 = 250;
pub const BULLET_UPDATE_MS: u32 = 50;

impl EntityAction for Entity {
    fn hit(&self, pos: Position, dims: Dimensions) -> bool {
        match self {
            Entity::Player(player) => player.hit(pos, dims),
            Entity::Rat(rat) => rat.hit(pos, dims),
            Entity::Brat(brat) => brat.hit(pos, dims),
            Entity::Factory(factory) => factory.hit(pos, dims),
            Entity::Bullet(bullet) => bullet.hit(pos, dims),
        }
    }
    fn explode(&mut self) {
        match self {
            Entity::Player(player) => player.explode(),
            Entity::Rat(rat) => rat.explode(),
            Entity::Brat(brat) => brat.explode(),
            Entity::Factory(factory) => factory.explode(),
            Entity::Bullet(bullet) => bullet.explode(),
        }
    }
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

pub type State = u8;
pub mod state {
    pub const ALIVE: u8 = 0;
    pub const EXPLODING1: u8 = 1;
    pub const EXPLODING2: u8 = 2;
    pub const EXPLODING3: u8 = 3;
    pub const DEAD: u8 = 4;
}
