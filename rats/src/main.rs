use clap::Parser;
use maze::Maze;
use std::cmp::{max, min};
use video::{InitOptions, Result, Video};

mod maze;

#[derive(Parser)]
struct RatsOpts {
    /// Display index
    #[clap(short = 'd', long = "display")]
    display: Option<u32>,

    /// Window width
    #[clap(long = "window-width", alias = "w-wt")]
    window_width: Option<u32>,

    /// Window height
    #[clap(long = "window_height", alias = "w-ht")]
    window_height: Option<u32>,

    /// Maze width
    #[clap(long = "maze-width", alias = "m-wt")]
    maze_width: Option<usize>,

    /// Maze height
    #[clap(long = "maze-height", alias = "m-ht")]
    maze_height: Option<usize>,

    /// Scale factor (1 to 4)
    #[clap(short = 's', long = "scale")]
    scale: Option<u32>,
}

fn main() {
    if let Err(error) = play(RatsOpts::parse()) {
        println!("{error}");
    }
}

fn play(opts: RatsOpts) -> Result<()> {
    let mut video = video::init(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.window_height)
            .window_width(opts.window_width)
            .scale(opts.scale),
    )?;
    video.init_charmap()?;

    let (maze_width, maze_height) = maze_dimensions(&opts, &video);
    let _maze = Maze::new(maze_width, maze_height);

    Ok(())
}

fn maze_dimensions(opts: &RatsOpts, video: &Video) -> (usize, usize) {
    (
        //width
        if let Some(width) = opts.maze_width {
            min(width, 15)
        } else {
            max(video.cols(), 15)
        },
        // height
        if let Some(height) = opts.maze_height {
            min(height, 30)
        } else {
            max(video.rows(), 30)
        },
    )
}
