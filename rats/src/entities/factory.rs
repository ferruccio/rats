use super::{
    dir, state, Dimensions, Entity, EntityAction, Position, Rat, State,
    FACTORY_UPDATE_MS,
};
use crate::{
    game_context::{random, Action},
    maze::Maze,
};
use video::{
    SizeWrapping, ATTR_NONE, BIG_BLANK_START, BIG_BOOM_A1, BIG_BOOM_A2,
    FACTORY_A1, FACTORY_A2,
};

#[derive(Debug, Clone, Copy)]
pub struct Factory {
    pub update: u32,
    pub pos: Position,
    pub state: State,
    pub cycle: u8,
}

impl EntityAction for Factory {
    fn hit(&self, pos: Position, dims: Dimensions) -> bool {
        if self.state != state::ALIVE {
            return false;
        }
        if pos == self.pos {
            return true;
        }
        let row_1 = self.pos.row.inc(dims.rows);
        let col_1 = self.pos.col.inc(dims.cols);
        pos == Position {
            row: self.pos.row,
            col: col_1,
        } || pos
            == Position {
                row: row_1,
                col: self.pos.col,
            }
            || pos
                == Position {
                    row: row_1,
                    col: col_1,
                }
    }

    fn explode(&mut self) {
        self.state = state::EXPLODING1;
    }
}

pub fn render_factory(factory: &Factory, maze: &mut Maze) {
    let ch = match factory.state {
        state::ALIVE => {
            if (factory.cycle & 1) == 0 {
                FACTORY_A1
            } else {
                FACTORY_A2
            }
        }
        state::EXPLODING1 => BIG_BOOM_A1,
        state::EXPLODING2 => BIG_BOOM_A2,
        state::EXPLODING3 => BIG_BOOM_A1,
        state::DEAD | _ => BIG_BLANK_START,
    };
    maze.buffer
        .set_quad(factory.pos.row, factory.pos.col, ch, ATTR_NONE);
}

pub fn update_factory(
    factory: &Factory,
    update: u32,
    make_rat: bool,
) -> Action {
    if update < factory.update + FACTORY_UPDATE_MS {
        return Action::Nothing;
    }
    let factory = *factory;
    match factory.state {
        state::ALIVE => {
            if make_rat {
                Action::New(Entity::Rat(Rat {
                    update,
                    distance: random(5, 15),
                    pos: factory.pos,
                    dir: dir::RIGHT,
                    state: state::ALIVE,
                    cycle: 0,
                }))
            } else {
                Action::Update(Entity::Factory(Factory {
                    update: update + FACTORY_UPDATE_MS,
                    cycle: (factory.cycle + 1) & 0x1,
                    ..factory
                }))
            }
        }
        state::EXPLODING1 => Action::Update(Entity::Factory(Factory {
            update: update + FACTORY_UPDATE_MS / 2,
            state: state::EXPLODING2,
            ..factory
        })),
        state::EXPLODING2 => Action::Update(Entity::Factory(Factory {
            update: update + FACTORY_UPDATE_MS / 2,
            state: state::EXPLODING3,
            ..factory
        })),
        state::EXPLODING3 => Action::Update(Entity::Factory(Factory {
            update: update + FACTORY_UPDATE_MS / 2,
            state: state::DEAD,
            ..factory
        })),
        state::DEAD | _ => Action::Delete,
    }
}
