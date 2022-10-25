# Hilbert Curve Algorithm

[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/hilbert_curve_rust-8dagcb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/hilbert-curve-rust)
[<img alt="crates.io" src="https://img.shields.io/crates/v/hilbert_curve_rust.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/hilbert-curve-rust)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.hilbert_curve_rust-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/starter_project)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/mrdesjardins/hilbert-curve-rust/Build/master?style=for-the-badge" height="20">](https://github.com/mrdesjardins/hilbert-curve-rust/actions?query=branch%3Amaster)


Rust implentation of the Hilbert Curve algoritm. The library moves from point (x, y) to index (z) and index (z) to point (x, y).

# As a Consumer of the Library
## Install

```sh
cargo add hilbert-curve-rust
```
Detail on [Crate.io](https://crates.io/crates/hilbert-curve-rust)

# #How to use?


### Index to point

```rust
let hilbert_curve = HilbertCurveAlgorithm::new(1); // Set the Hilbert order here
let point = hilbert_curve.index_to_point(0); // Get the point for index 0
```

### Point to index

```rust
let hilbert_curve = HilbertCurveAlgorithm::new(1);// Set the Hilbert order here
let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 0 }); // Get the index for (0,0) point
```

# As a Developer of the library
If you want to contribute to this code base. Here are few informations that might be useful.
## Test and Coverage

### Coverage

You must install few components before running coverage:

```sh
cargo install grcov
rustup component add llvm-tools-preview
```
Then, you can run:
```sh
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="hilbertcurve-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```
Further explanation in the [Mozilla grcov website](https://github.com/mozilla/grcov)

## Documentation

```sh
cargo doc --open
```
