# Harter Heighway Dragon Curve

In this project I used Rust to generate the Harter Heighway Dragon Curve using a simple iterative method based on rotating and appending direction vectors.

## How It Works

The method used to build is the: Iterated Geometric Folding (Rotation + Reflection Method)

The steps of the method are the following:

1. Start with one direction (e.g. right → (1, 0)).
2.	At each step:
   - Reverse the current list of vectors.
   - Rotate them 90° (usually clockwise).
   - Append them to the original list.
3.	Use cumulative sum to get all coordinates.

## Build & Run

**No optimizations**
```bash
cargo run
```

**Optimized**
```bash
cargo run --release
```

You can change the number of iterations directly in `main.rs`:
```rust
// Number of iterations
const ITERS: usize = 20;
```

## File Overview
- `main` $\rightarrow$ main loop and variable initializations
- `generate_directions` $\rightarrow$ builds the list of direction vectors one steps at time
- `generate_coord` $\rightarrow$ builds the list of coordinates from the vectors one step at time
- `draw` $\rightarrow$ draws the pixels in the window buffer

## License

[MIT](https://choosealicense.com/licenses/mit/)

