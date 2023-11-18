# Rats

This is an attempt to re-create the game of Rats, which I first played more than
forty years ago on a Convergent Technologies IWS Workstation.

After a little searching, I ran across
[this video](https://www.youtube.com/watch?v=CBqMuL_LlP4) which has really
helped to jog my memory in terms of both graphical appearance and gameplay
mechanics.

## Gameplay

The game's controls are pretty simple.

- Arrow keys control player motion. You can move on a diagonal by pressing two
  keys at the same time. e.g. Holding down both up and right arrows will move
  the player up and to the right at the same time.
- WASD keys control firing. You can shoot in a different direction than the
  direction you're moving in. You can also shoot on diagonals by holding down
  two firing keys at the same time.
- Space bar pauses game.
- Esc exits game.

Rats has a number of command-line options which let you set maze size and
density and a few other parameters. My original intent was to emulate the look
of the game as close as possible, but I eventually took some liberties like
adding a little color. You can use the `--classic` option to make the game look
and play more like the original.

Run `rats --help` for a more detailed list of options.

The sound effects came from [kronbits](https://kronbits.itch.io/freesfx)

## Installation

### Requirements

- [Rust compiler](https://www.rust-lang.org/)
- Development version of SDL2. On an Ubuntu-based system, you can install this
  with the command:
  ```sh
  sudo apt install libsdl2-dev
  sudo apt install libsdl2-mixer-dev
  ```

then you can install the game:

```sh
cargo install --git https://github.com/ferruccio/rats.git rats
```

Theoretically, this thing should run on macOS and Windows as well, but I don't
have the time to try to tackle either of those right now.
