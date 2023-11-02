use super::{dir, state, Entity, Position, Rat, State};
use crate::{
    game_context::{random, Action},
    maze::Maze,
};
use video::{ATTR_NONE, FACTORY_A1, FACTORY_A2};

#[derive(Debug, Clone, Copy)]
pub struct Factory {
    pub updated: u32,
    pub pos: Position,
    pub state: State,
    pub cycle: u8,
}

pub fn render_factory(factory: &Factory, maze: &mut Maze) {
    let ch = if (factory.cycle & 1) == 0 {
        FACTORY_A1
    } else {
        FACTORY_A2
    };
    maze.buffer
        .set_quad(factory.pos.row, factory.pos.col, ch, ATTR_NONE);
}

// frames per unit of factory animation
const FACTORY_FRAMES: u32 = 15;

pub fn update_factory(
    factory: &Factory,
    frames: u32,
    make_rat: bool,
) -> Action {
    if frames < factory.updated + FACTORY_FRAMES {
        Action::Nothing
    } else {
        match factory.state {
            state::ALIVE => {
                if make_rat {
                    return Action::New(Entity::Rat(Rat {
                        updated: frames,
                        distance: random(5, 15),
                        pos: factory.pos,
                        dir: dir::RIGHT,
                        state: state::ALIVE,
                        cycle: 0,
                    }));
                }
            }
            state::EXPLODING1 => {}
            state::EXPLODING2 => {}
            state::EXPLODING3 => {}
            state::DEAD => {}
            _ => {}
        }
        Action::Update(Entity::Factory(Factory {
            updated: frames + FACTORY_FRAMES,
            cycle: (factory.cycle + 1) & 0x1,
            ..*factory
        }))
    }
}
