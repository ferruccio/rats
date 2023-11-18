use super::{dir, Entity, EntityAction, Position, Rat, State};
use crate::{
    config::FACTORY_UPDATE_MS,
    game_context::{random, Action},
    maze::{with_pristine_maze, Maze},
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
    fn hit(&self, pos: Position) -> bool {
        if self.state != State::Alive {
            return false;
        }
        if pos == self.pos {
            return true;
        }
        let (rows, cols) =
            with_pristine_maze(|maze| (maze.rows(), maze.cols()));
        let row_1 = self.pos.row.inc(rows);
        let col_1 = self.pos.col.inc(cols);
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
        self.state = State::Exploding1;
    }
}

pub fn render_factory(factory: &Factory, maze: &mut Maze) {
    let ch = match factory.state {
        State::Alive => {
            if (factory.cycle & 1) == 0 {
                FACTORY_A1
            } else {
                FACTORY_A2
            }
        }
        State::Exploding1 => BIG_BOOM_A1,
        State::Exploding2 => BIG_BOOM_A2,
        State::Exploding3 => BIG_BOOM_A1,
        State::Dead => BIG_BLANK_START,
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
        State::Alive => {
            if make_rat {
                Action::New(Entity::Rat(Rat {
                    update,
                    distance: random(5, 15),
                    pos: factory.pos,
                    dir: dir::RIGHT,
                    state: State::Alive,
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
        State::Exploding1 => Action::Update(Entity::Factory(Factory {
            update: update + FACTORY_UPDATE_MS / 2,
            state: State::Exploding2,
            ..factory
        })),
        State::Exploding2 => Action::Update(Entity::Factory(Factory {
            update: update + FACTORY_UPDATE_MS / 2,
            state: State::Exploding3,
            ..factory
        })),
        State::Exploding3 => Action::Update(Entity::Factory(Factory {
            update: update + FACTORY_UPDATE_MS / 2,
            state: State::Dead,
            ..factory
        })),
        State::Dead => Action::Delete,
    }
}
