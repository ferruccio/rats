use super::GameContext;
use crate::{
    entities::{Dimensions, Entity, Factory, Position, State},
    maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS},
};
use rand::{distributions::Uniform, thread_rng, Rng};
use std::{cmp::min, time::Instant};
use video::{Size, SizeWrapping};

impl GameContext {
    pub fn generate_factories(&mut self, count: usize, maze: &Maze) {
        let mut rng = thread_rng();
        let row_distribution = Uniform::new_inclusive(0, maze.rows() - 1);
        let col_distribution = Uniform::new_inclusive(0, maze.cols() - 1);
        let mut generated = 0;
        let mut positions = vec![];
        let player_pos = Position {
            row: MAZE_CELL_ROWS / 2,
            col: MAZE_CELL_COLS / 2,
        };
        let time = Instant::now();
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
            if distance_squared(pos, player_pos, maze.dimensions) < 250 {
                continue 'again;
            }
            for p in positions.iter() {
                if distance_squared(pos, *p, maze.dimensions) < 25 {
                    continue 'again;
                }
            }
            positions.push(pos);
            self.entities.push(Entity::Factory(Factory {
                updated: self.frames,
                pos,
                state: State::Alive,
                cycle: 0,
            }));
            generated = generated + 1;
        }
    }
}

// square of the distance between two points on a torus
fn distance_squared(pos1: Position, pos2: Position, dims: Dimensions) -> Size {
    let x1 = pos1.col as i32;
    let x2 = pos2.col as i32;
    let y1 = pos1.row as i32;
    let y2 = pos2.row as i32;
    let w = dims.cols as i32;
    let h = dims.rows as i32;
    // min(|x1 - x2|, w - |x1 - x2|)^2 + min(|y1 - y2|, h - |y1 - y2|)^2
    let mx = min((x1 - x2).abs(), w - (x1 - x2).abs());
    let my = min((y1 - y2).abs(), h - (y1 - y2).abs());
    (mx * mx + my * my) as Size
}
