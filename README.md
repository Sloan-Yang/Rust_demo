# Rust Demo

![Alien Cake Chase screenshot](assets/image.jpg)

## Overview
- 3D mini game built with Bevy 0.15 where a hungry alien chases rotating cakes across a floating grid.
- Procedurally offsets tile heights and animates lighting for a playful diorama feel.
- Smooth camera focus that lerps between the player and the current cake and eases back to the board center when nothing is in focus.
- State-driven gameplay loop (`Playing` ↔ `GameOver`) keeps UI, entities, and systems tidy.

## How to Play
- `Arrow keys` move the alien across the board.
- Collect the cake before it respawns elsewhere to gain **+2** score.
- Miss the cake spawn and you lose **-3**; hitting **-5** ends the run.
- `Space` restarts after a game over.

## Getting Started
1. Install the latest stable [Rust toolchain](https://www.rust-lang.org/tools/install) (Bevy works best on recent stable releases).
2. Fetch dependencies and run the game:
   ```bash
   cargo run
   ```
3. Optional: export `GITHUB_ACTION=true` to force deterministic RNG (useful for automated captures/tests).

## Project Layout
- `src/main.rs` – game state, systems, and Bevy app wiring.
- `assets/models/AlienCake/*.glb` – board tile, player, and cake models.
- `assets/image.jpg` – reference render used above.
- `backup.rs` – experimental scratchpad with earlier logic (not compiled).

## Development Notes
- Gameplay constants live near the top of `src/main.rs` (`BOARD_SIZE_*`, spawn timers, camera focus).
- Systems lean on `StateScoped` entities to spawn/cleanup automatically when transitioning between states.
- The RNG resource wraps `ChaCha8Rng`, swapping seeds when `GITHUB_ACTION` is set to keep CI runs reproducible.

## Next Ideas
- Add audio feedback when grabbing a cake or losing points.
- Introduce hazards or power-ups that tweak the score multiplier.
- Surface high scores and streaks to encourage multiple runs.
