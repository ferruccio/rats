use crate::{
    entities::{render_bullet, render_player, Entity},
    game_context::GameContext,
    maze::Maze,
    Result,
};
use sdl2::render::Texture;
use video::{ATTR_DIM, ATTR_REVERSE};

impl GameContext {
    pub fn render_frame(&mut self, textures: &[Texture]) -> Result<()> {
        // start with a clear video buffer and pristine maze
        self.video.buffer.clear();
        self.pristine_maze.buffer.copy_to(&mut self.maze.buffer);

        // render all entities onto our current maze
        for entity in self.entities.iter_mut() {
            render_entity(entity, &mut self.maze);
        }

        // copy the visible portion of our current maze into the video buffer
        let mut start_pos = self.player_position();
        start_pos
            .move_up((self.video.buffer.rows - 2) / 2, self.maze.dimensions);
        start_pos.move_left(self.video.buffer.cols / 2, self.maze.dimensions);
        self.maze.buffer.copy_buffer(
            start_pos.row,
            start_pos.col,
            &mut self.video.buffer,
            2,
        );
        self.frames += 1;

        // add status text to video buffer
        let seconds = self.start.elapsed().as_secs_f32();
        let fps =
            self.frames as f32 / if seconds == 0.0 { 1.0 } else { seconds };
        let player_pos = self.player_position();
        let maze_rows = self.maze.rows();
        let maze_cols = self.maze.cols();
        let entities = self.entities.len();
        let vbuf = &mut self.video.buffer;
        const RD: u8 = ATTR_REVERSE | ATTR_DIM;
        vbuf.print(5, 0, RD, format!("   FPS: {fps:.0}"));
        vbuf.print(6, 0, RD, format!("  maze: {maze_rows} x {maze_cols}",));
        vbuf.print(7, 0, RD, format!("player: {player_pos}"));
        vbuf.print(8, 0, RD, format!(" start: {start_pos}"));
        vbuf.print(9, 0, RD, format!("  ents: {entities}"));
        // blast the video buffer onto the screen
        self.video.render_buffer(textures)
    }
}

fn render_entity(entity: &Entity, maze: &mut Maze) {
    match entity {
        Entity::Player(player) => render_player(player, maze),
        Entity::_Rat(_rat) => todo!(),
        Entity::_BabyRat(_baby_rat) => todo!(),
        Entity::_RatFactory(_rat_factory) => todo!(),
        Entity::Bullet(bullet) => render_bullet(bullet, maze),
    }
}
