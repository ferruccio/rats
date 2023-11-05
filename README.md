# Rats

This is an attempt to re-create the game of Rats, which I first played more than
forty years ago on a Convergent Technologies IWS Workstation.

After a little searching, I ran across
[this video](https://www.youtube.com/watch?v=CBqMuL_LlP4) which has helped to
jog my memory in terms of both graphical appearance and game-play mechanics.

## Gameplay

The game is playable but very incomplete right now.

- Arrow keys control player motion.
- WASD kets control firing.
- Space bar pauses game.
- Esc exits game.

Run `rats --help` for command-line options.

## Installation

### Requirements

- [Rust compiler](https://www.rust-lang.org/)
- Development version of SDL2. On an Ubuntu-based system, you can install this
  with the command:
  ```sh
  sudo apt install libsdl2-dev
  ```

then you can install the game:

```sh
cargo install --git https://github.com/ferruccio/rats.git rats
```

Theoretically, this thing should run on macOS and Windows as well, but I don't
have the time (or the will) to try either one of those at this time.
