use super::GameContext;
use crate::{
    entities::{dir, Dimensions, Direction, Entity, Factory, Position, State},
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
            if distance_squared(pos, player_pos, maze.dimensions) < 225 {
                continue 'again;
            }
            // factories must be at least 5 characters away from other factories
            for p in positions.iter() {
                if distance_squared(pos, *p, maze.dimensions) < 25 {
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

// square of the distance between two points on a torus
pub fn distance_squared(
    pos1: Position,
    pos2: Position,
    dims: Dimensions,
) -> Size {
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

impl Position {
    pub fn direction_to(&self, pos: Position, dims: Dimensions) -> Direction {
        let mut pos_up = self.clone();
        pos_up.move_up(1, dims);
        let dist_up = distance_squared(pos_up, pos, dims);
        let mut pos_down = self.clone();
        pos_down.move_down(1, dims);
        let dist_down = distance_squared(pos_down, pos, dims);
        let mut pos_left = self.clone();
        pos_left.move_left(1, dims);
        let dist_left = distance_squared(pos_left, pos, dims);
        let mut pos_right = self.clone();
        pos_right.move_right(1, dims);
        let dist_right = distance_squared(pos_right, pos, dims);

        if dist_up < dist_down && dist_up < dist_left && dist_up < dist_right {
            dir::UP
        } else if dist_down < dist_left && dist_down < dist_right {
            dir::DOWN
        } else if dist_left < dist_right {
            dir::LEFT
        } else {
            dir::RIGHT
        }
    }
}
