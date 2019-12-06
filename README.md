Random Process Project 2019
===

## Quick Start
### 1. cargo build

Fetch necessary library and compile program (require network connection and take a bit of time)

### 2. (RUST_LOG=debug) cargo run

Enable debug message and run

### 3. (Optional) mkdir out
### 4. cd plot
### 5. python [plot_a.py / plot_b.py / ...]

## Advanced Usage

Currently there is no user argument input parser for an interactive script, so the user may have to manually twick the `const` in rust file in order to have different functionality.

1. Move Starting point

Modify `PoA`/`PoB`/`PoC` defined in src/main.rs to move the starting point, by default it will be 30, 30, 30

2. Extend Experiment Times

Modify `TRANS_THRES` to specify the maximum step in one game (in order to prevent potential infinite loop), by default it will be 10000
Modify NTHREAD to extend how many round of game will be play, by default it will be 10000

3. Problem 3

If you want to move on to problem 3, you will have to change the value of `Problem3` to true, by default it will be false