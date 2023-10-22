use sdl2::surface::Surface;

pub struct Font<'f> {
    pub surface: Surface<'f>,
}

impl<'f> Font<'f> {
    pub fn new(surface: Surface) -> Font {
        Font { surface }
    }
}
