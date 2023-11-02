use crate::{
    entities::{update_bullet, update_player, Entity},
    game_context::GameContext,
};

pub enum Action {
    Nothing,
    Delete,
    Update(Entity),
}

impl GameContext {
    pub fn update(&mut self) {
        let mut actions: Vec<(usize, Action)> = vec![];
        for (index, entity) in self.entities.iter().enumerate() {
            let action = match entity {
                Entity::Player(player) => {
                    update_player(&player, &self.pristine_maze, self.frames)
                }
                Entity::_Rat(_) => Action::Nothing,
                Entity::_BabyRat(_) => Action::Nothing,
                Entity::_RatFactory(_) => Action::Nothing,
                Entity::Bullet(bullet) => {
                    update_bullet(&bullet, &self.pristine_maze, self.frames)
                }
            };
            actions.push((index, action));
        }
        for (index, action) in actions.into_iter().rev() {
            match action {
                Action::Nothing => {}
                Action::Delete => {
                    let last = self.entities.len() - 1;
                    self.entities.swap(index, last);
                    self.entities.truncate(last);
                }
                Action::Update(entity) => {
                    self.entities[index] = entity;
                }
            };
        }
    }
}
