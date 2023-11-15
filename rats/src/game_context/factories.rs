use super::GameContext;
use crate::{
    entities::{Entity, Factory, Position, State},
    maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS},
};
use rand::{distributions::Uniform, thread_rng, Rng};
use std::time::Instant;
use video::SizeWrapping;

impl GameContext {
    pub fn generate_factories(&mut self, count: usize, maze: &Maze) {
        let mut rng = thread_rng();
        let row_distribution = Uniform::new_inclusive(0, maze.rows() - 1);
        let col_distribution = Uniform::new_inclusive(0, maze.cols() - 1);
        let cycle_distribution = Uniform::new_inclusive(0, 1);
        let mut generated = 0;
        let mut positions = vec![];
        // we know where the player starts
        let player_pos = Position {
            row: MAZE_CELL_ROWS / 2,
            col: MAZE_CELL_COLS / 2,
        };
        let time = Instant::now();
        // don't spend more than a second on this
        'again: while generated < count && time.elapsed().as_millis() < 1000 {
            let row1 = rng.sample(row_distribution);
            let col1 = rng.sample(col_distribution);
            let row2 = row1.inc(maze.rows());
            let col2 = col1.inc(maze.cols());
            if maze.is_wall(row1, col1)
                || maze.is_wall(row1, col2)
                || maze.is_wall(row2, col1)
                || maze.is_wall(row2, col2)
            {
                continue 'again;
            }
            let pos = Position {
                row: row1,
                col: col1,
            };
            // factories must be at least 15 characters away from the player
            if pos.distance_squared_to(player_pos, maze.dimensions) < 225 {
                continue 'again;
            }
            // factories must be at least 5 characters away from other factories
            for p in positions.iter() {
                if pos.distance_squared_to(*p, maze.dimensions) < 25 {
                    continue 'again;
                }
            }
            positions.push(pos);
            self.entities.push(Entity::Factory(Factory {
                update: self.elapsed(),
                pos,
                state: State::Alive,
                cycle: rng.sample(cycle_distribution),
            }));
            generated += 1;
        }
        self.live_factories = generated;
    }
}
