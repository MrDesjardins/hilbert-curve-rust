# Hilbert Curve Algorithm

Rust implentation of the Hilbert Curve algoritm. The library allows to move from point to index and index to point.

# How to use?

## Index to point

```rust
let hilbert_curve = HilbertCurveAlgorithm::new(1); // Set the Hilbert order here
let point = hilbert_curve.index_to_point(0); // Get the point for index 0
```

## Point to index

```rust
let hilbert_curve = HilbertCurveAlgorithm::new(1);// Set the Hilbert order here
let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 0 }); // Get the index for (0,0) point
```

# Test and Coverage

## Coverage

```sh
rustup component add llvm-tools-preview
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```
Further explanation in the [Mozilla grcov website](https://github.com/mozilla/grcov)

# Documentation

```sh
cargo doc --open
```
