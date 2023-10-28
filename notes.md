38-oct-2023 - week 1

- Decided to use SDL2 to simulate a text-mode UI.
- Characters will be stored in an 8x12 pixel bitmap directly in the source code.
- On startup, we will take that 8x12 bitmap and generate an SDL surface where
  each character has been pre-rendered.
- Each character will be represented by one byte. Bytes 0x00 through 0x7f will
  represent the usual ASCII characters. The remaining characters will be used to
  represent custom game characters. These characters will be rendered in a
  bright green. Any undefined characters will be rendered as a bright red dot.
- Command line options control display selection, window size, maze size and
  scale factor.
- Support character attributes. We only need to support reverse video & dimmed
  characters.
- Don't rely on keyboard repeat to move player.
- Create wall characters.
- Generate and render maze.

This came together pretty quickly. The only real issues seemed to always involve getting the arithmetic right. There was a lot
of headscratching while getting the character bitmaps to render correctly; and some more getting the relationship between the
maze and the screen buffer right.

At first I was using both height/width and rows/cols terminology to represent the dimensions of the maze and the screen buffer.
I also used `u32`, `usize`, `i32` and `isize` as I saw fit in the moment. I created a `Pixels` type alias (`usize`)to represent window
and surface bitmap dimensions (expressed as height/width) and a `Chars` type alias (`usize`) to represent maze and screen buffer dimensions
(in rows/cols). Adopting this convention flushed out some of the existing problems.
