# Flappy Ball Game

A fun endless high-score game built using Rust and `ggez`.

## Gameplay Design
![Flappy Ball Demo](assets/design.jpeg)

## Controls
- **SPACE**: Jump
- **P**: Pause/Resume game
- **R**: Restart game

## How to Run
1. Clone the repository.
2. Use `cargo run` or `make run` to start the game.
3. Use `make test` to run the unit tests.
4. Use `make clean` to remove all target files.

## Positive Test Cases
1. Ball Movement
2. Pipe Generation
3. Score track

## Negative Test Cases
1. No score increment without passing pipe
2. Ball collision with pipe
3. Ball out of bound

## Author
Sandeep Chikkapla Siddappa
