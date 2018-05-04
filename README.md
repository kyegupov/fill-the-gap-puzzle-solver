# fill-the-gap-puzzle-solver

Solver for a game "fill the gaps in the board with pieces, with minimum overlaps".

Place the pieces on the board to fill the gaps. You are penalized each time a piece
overlaps with the board or other piece, so your score is always negative.

A very naive bruteforce combo that builds all the possible placements of pieces and finds 
the best.

# How to build and run

Install Rust (https://www.rustup.rs/ - or on Linux, `curl https://sh.rustup.rs -sSf | sh -s`)

Then just `cargo run --release`

Building from scratch will take about a minute. Be patient. It's the Rust way.

Running in non-release mode is not recommended (it's literally 100 times slower).
