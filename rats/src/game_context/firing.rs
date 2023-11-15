use std::time::Instant;

use super::GameContext;
use crate::entities::{
    dir, Bullet, Direction, Entity, EntityAction, Position, State,
};
use video::SizeWrapping;

impl GameContext {
    pub fn fire(&mut self) {
        let player = self.get_player();
        if player.state != State::Alive || self.next_fire_time > Instant::now()
        {
            return;
        }
        let dir = self.effective_firing_dir();
        let (row, col) = (player.pos.row, player.pos.col);
        let (rows, cols) = (self.maze.rows(), self.maze.cols());
        if let Some((row, col)) = match dir {
            dir::DOWN => Some((row.inc(rows).inc(rows), col)),
            dir::DOWN_LEFT => Some((row.inc(rows), col.dec(cols))),
            dir::DOWN_RIGHT => Some((row.inc(rows), col.inc(cols).inc(cols))),
            dir::UP => Some((row.dec(rows), col.inc(cols))),
            dir::UP_LEFT => Some((row.dec(rows), col.dec(cols))),
            dir::UP_RIGHT => Some((row.dec(rows), col.inc(cols).inc(cols))),
            dir::LEFT => Some((row, col.dec(cols))),
            dir::RIGHT => Some((row, col.inc(cols).inc(cols))),
            _ => None,
        } {
            if self.maze.is_wall(row, col) {
                return;
            }
            self.bullet_fire_start = Instant::now();
            self.next_fire_time =
                self.bullet_fire_start + self.bullet_firing_time;
            let pos = Position { row, col };
            for entity in self.entities.iter_mut().skip(1) {
                if entity.hit(pos) {
                    entity.explode();
                    return;
                }
            }
            self.entities.push(Entity::Bullet(Bullet {
                update: self.elapsed(),
                pos,
                dir,
                state: State::Alive,
            }));
        }
    }

    pub fn start_firing(&mut self, dir: Direction) {
        if self.firing_dir & dir == 0 {
            self.firing_dir |= dir;
            self.fire();
        }
    }

    pub fn stop_firing(&mut self, dir: Direction) {
        self.firing_dir &= !dir;
    }

    pub fn effective_firing_dir(&self) -> Direction {
        let mut dir = self.firing_dir;
        if (dir & dir::UP) != 0 && (dir & dir::DOWN) != 0 {
            dir &= !(dir::UP | dir::DOWN);
        }
        if (dir & dir::LEFT) != 0 && (dir & dir::RIGHT) != 0 {
            dir &= !(dir::LEFT | dir::RIGHT);
        }
        dir
    }
}
