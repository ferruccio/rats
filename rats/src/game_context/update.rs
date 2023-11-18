use super::GameState;
use crate::{
    config::{BRAT_KILL, FACTORY_KILL, RAT_KILL},
    entities::{
        update_brat, update_bullet, update_factory, update_player, update_rat,
        Entity, EntityAction, State,
    },
    game_context::GameContext,
};

pub enum Action {
    Nothing,
    Delete,
    Update(Entity),
    New(Entity),
    Attack(usize),
}

impl GameContext {
    pub fn update(&mut self) {
        if self.game_state == GameState::Running {
            let actions = self.update_actions();
            self.apply_actions(actions);
            self.bullet_hit_tests();
        }
    }

    fn update_actions(&self) -> Vec<(usize, Action)> {
        let update = self.elapsed();
        let mut actions: Vec<(usize, Action)> = vec![];
        for (index, entity) in self.entities.iter().enumerate() {
            let action = match entity {
                Entity::Player(player) => update_player(player, update),
                Entity::Rat(rat) => update_rat(
                    rat,
                    self.get_player(),
                    self.rat_damage,
                    update,
                    self.new_brats != 0,
                ),
                Entity::Brat(brat) => update_brat(
                    brat,
                    self.get_player(),
                    self.brat_damage,
                    update,
                ),
                Entity::Factory(factory) => {
                    update_factory(factory, update, self.new_rats != 0)
                }
                Entity::Bullet(bullet) => update_bullet(bullet, update),
            };
            actions.push((index, action));
        }
        actions
    }

    fn apply_actions(&mut self, actions: Vec<(usize, Action)>) {
        for (index, action) in actions.into_iter().rev() {
            match action {
                Action::Nothing => {}
                Action::Delete => {
                    match self.entities[index] {
                        Entity::Player(_) => {}
                        Entity::Rat(_) => {
                            self.live_rats -= 1;
                            self.dead_rats += 1;
                        }
                        Entity::Brat(_) => {
                            self.live_brats -= 1;
                            self.dead_brats += 1;
                        }
                        Entity::Factory(_) => {
                            self.live_factories -= 1;
                            self.dead_factories += 1;
                        }
                        Entity::Bullet(_) => {}
                    }
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
                Action::Attack(damage) => {
                    self.entities[index].explode();
                    if damage >= self.health {
                        if self.players_left > 0 {
                            self.entities[0].explode();
                            self.players_left -= 1;
                            self.players_dead += 1;
                        }
                    } else {
                        self.health -= damage;
                    }
                }
            };
        }
    }

    fn bullet_hit_tests(&mut self) {
        let live_bullets: Vec<_> = self
            .entities
            .iter()
            .enumerate()
            .filter_map(|(index, entity)| match entity {
                Entity::Bullet(bullet) if bullet.state == State::Alive => {
                    Some((index, bullet.pos, bullet.lifetime))
                }
                _ => None,
            })
            .collect();
        let mut marks = vec![false; self.entities.len()];
        for (bullet_index, pos, lifetime) in live_bullets.into_iter().rev() {
            for (entity_index, entity) in self.entities.iter_mut().enumerate() {
                if entity.hit(pos) && bullet_index != entity_index {
                    match entity {
                        Entity::Player(player) => {
                            if lifetime > 10 {
                                self.super_boom = 60;
                                self.players_dead += 1;
                                self.players_left -= 1;
                                player.explode();
                            }
                        }
                        Entity::Rat(rat) => {
                            self.score += RAT_KILL;
                            rat.explode();
                        }
                        Entity::Brat(brat) => {
                            self.score += BRAT_KILL;
                            brat.explode();
                        }
                        Entity::Factory(factory) => {
                            self.super_boom = 60;
                            self.score += FACTORY_KILL;
                            factory.explode();
                        }
                        Entity::Bullet(bullet) => {
                            bullet.explode();
                        }
                    }
                    marks[bullet_index] = true;
                }
            }
        }
        for (index, marked) in marks.into_iter().enumerate().rev() {
            if marked {
                let last = self.entities.len() - 1;
                if index != last {
                    self.entities.swap(index, last);
                }
                self.entities.truncate(last);
            }
        }
    }
}
