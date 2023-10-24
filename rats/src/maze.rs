pub struct Maze {
    _char_width: usize,
    _char_height: usize,
    _buffer: Vec<u8>,
}

// maze cell dimensions in characters
const MAZE_CELL_HEIGHT: usize = 5;
const MAZE_CELL_WIDTH: usize = 10;

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        if width < 2 || height < 2 {
            panic!("invalid maze dimensions");
        }
        let char_width = (MAZE_CELL_WIDTH + 1) * width;
        let char_height = (MAZE_CELL_HEIGHT + 1) * height;
        Maze {
            _char_height: char_height,
            _char_width: char_width,
            _buffer: vec![b' '; height * width],
        }
    }
}
