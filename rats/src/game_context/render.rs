use crate::{
    entities::{
        render_brat, render_bullet, render_factory, render_player, render_rat,
        Entity,
    },
    game_context::GameContext,
    maze::Maze,
    Result,
};
use sdl2::render::Texture;
use video::{ATTR_NONE, BRATS_UP_A1, FACTORY_A2, PLAYER_DOWN, RATS_UP_A1};

impl GameContext {
    pub fn render_frame(&mut self, textures: &[Texture]) -> Result<()> {
        // start with a clear video buffer and pristine maze
        self.video.buffer.clear();
        self.pristine_maze.buffer.copy_to(&mut self.maze.buffer);

        // render all entities in reverse order so that player
        // and rat factories are rendered last
        for entity in self.entities.iter_mut().rev() {
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

        // uncomment code to render diagnostics
        /*
        let seconds = self.start.elapsed().as_secs_f32();
        let fps =
            self.frames as f32 / if seconds == 0.0 { 1.0 } else { seconds };
        let player_pos = self.player_position();
        let maze_rows = self.maze.rows();
        let maze_cols = self.maze.cols();
        let entities = self.entities.len();
        */
        let vbuf = &mut self.video.buffer;
        /*
        const RD: u8 = video::ATTR_REVERSE | video::ATTR_DIM;
        vbuf.print(2, 0, RD, format!("   FPS: {fps:.0}"));
        vbuf.print(3, 0, RD, format!("  maze: {maze_rows} x {maze_cols}",));
        vbuf.print(4, 0, RD, format!("player: {player_pos}"));
        vbuf.print(5, 0, RD, format!(" start: {start_pos}"));
        vbuf.print(6, 0, RD, format!("  ents: {entities}"));
        */
        let time = self.start.elapsed().as_secs();

        // rat stats
        vbuf.set_quad(0, 1, RATS_UP_A1, ATTR_NONE);
        vbuf.print(0, 3, ATTR_NONE, format!("{:3} dead", self.dead_rats));
        vbuf.print(1, 3, ATTR_NONE, format!("{:3} left", self.live_rats));
        // brat stats
        vbuf.set_chattr(0, 13, BRATS_UP_A1, ATTR_NONE);
        vbuf.print(0, 14, ATTR_NONE, format!("{:3} dead", self.dead_brats));
        vbuf.print(1, 14, ATTR_NONE, format!("{:3} left", self.live_brats));
        // factory stats
        vbuf.set_quad(0, 24, FACTORY_A2, ATTR_NONE);
        vbuf.print(0, 26, ATTR_NONE, format!("{:3} dead", self.dead_factories));
        vbuf.print(1, 26, ATTR_NONE, format!("{:3} left", self.live_factories));
        // player stats
        vbuf.set_quad(0, 36, PLAYER_DOWN, ATTR_NONE);
        vbuf.print(0, 39, ATTR_NONE, format!("{:1} dead", 0));
        vbuf.print(1, 39, ATTR_NONE, format!("{:1} left", 2));
        // game stats
        vbuf.print(0, 47, ATTR_NONE, format!("Score: {:4}", 0));
        vbuf.print(1, 47, ATTR_NONE, "High:     0");
        vbuf.print(0, 60, ATTR_NONE, format!("Time:  {:4}", time));
        vbuf.print(1, 60, ATTR_NONE, format!("Maze: {:5}", 32768));

        // blast the video buffer onto the screen
        self.video.render_buffer(textures)
    }
}

fn render_entity(entity: &Entity, maze: &mut Maze) {
    match entity {
        Entity::Player(player) => render_player(player, maze),
        Entity::Rat(rat) => render_rat(rat, maze),
        Entity::Brat(brat) => render_brat(brat, maze),
        Entity::Factory(factory) => render_factory(factory, maze),
        Entity::Bullet(bullet) => render_bullet(bullet, maze),
    }
}
