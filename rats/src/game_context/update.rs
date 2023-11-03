use crate::{
    entities::{
        update_brat, update_bullet, update_factory, update_player, update_rat,
        Entity,
    },
    game_context::GameContext,
};

pub enum Action {
    Nothing,
    Delete,
    Update(Entity),
    New(Entity),
}

impl GameContext {
    pub fn update(&mut self) {
        let mut actions: Vec<(usize, Action)> = vec![];
        for (index, entity) in self.entities.iter().enumerate() {
            let action = match entity {
                Entity::Player(player) => {
                    update_player(&player, &self.pristine_maze, self.frames)
                }
                Entity::Rat(rat) => update_rat(
                    &rat,
                    &self.pristine_maze,
                    self.frames,
                    self.new_brats != 0,
                ),
                Entity::Brat(brat) => {
                    update_brat(&brat, &self.pristine_maze, self.frames)
                }
                Entity::Factory(factory) => {
                    update_factory(&factory, self.frames, self.new_rats != 0)
                }
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
                Action::New(entity) => {
                    match entity {
                        Entity::Player(_) => {}
                        Entity::Rat(_) => {
                            self.live_rats += 1;
                            if self.new_rats > 0 {
                                self.new_rats -= 1;
                            }
                        }
                        Entity::Brat(_) => {
                            self.live_brats += 1;
                            if self.new_brats > 0 {
                                self.new_brats -= 1;
                            }
                        }
                        Entity::Factory(_) => self.live_factories += 1,
                        Entity::Bullet(_) => {}
                    }
                    self.entities.push(entity);
                }
            };
        }
    }
}
