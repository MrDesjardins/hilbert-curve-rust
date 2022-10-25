# Hilbert Curve Algorithm

[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/hilbert_curve_rust-8dagcb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/hilbert-curve-rust)
[<img alt="crates.io" src="https://img.shields.io/crates/v/hilbert_curve_rust.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/hilbert-curve-rust)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.hilbert_curve_rust-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/hilbert-curve-rust/latest/hilbert_curve_rust)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/mrdesjardins/hilbert-curve-rust/rust.yml?style=for-the-badge" height="20">](https://github.com/mrdesjardins/hilbert-curve-rust/actions?query=branch%3Amain)
[![codecov](https://codecov.io/gh/MrDesjardins/hilbert-curve-rust/branch/main/graph/badge.svg?token=TWHYC1X1KQ)](https://codecov.io/gh/MrDesjardins/hilbert-curve-rust)

Rust implentation of the Hilbert Curve algoritm. The library moves from point (x, y) to index (z) and index (z) to point (x, y).

# As a Consumer of the Library

## Install

```sh
cargo add hilbert-curve-rust
```

Detail on [Crate.io](https://crates.io/crates/hilbert-curve-rust)

## How to use?

The [Rust Documentation](https://docs.rs/hilbert-curve-rust/latest/hilbert_curve_rust) for the Hilbert Curve Rust library is available online. However, here are two examples to get started.

### Index to point

Single dimension integer into a two dimensionals coordinate

```rust
let hilbert_curve = HilbertCurveAlgorithm::new(1); // Set the Hilbert order here
let point = hilbert_curve.index_to_point(0); // Get the point for index 0
```

### Point to index

Two dimensionals coordinate into a ingle dimension integer.

```rust
let hilbert_curve = HilbertCurveAlgorithm::new(1);// Set the Hilbert order here
let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 0 }); // Get the index for (0,0) point
```

# As a Developer of the Hibert Curve Rust Library

If you want to contribute to the Hibert Curve Rust code base. Here are few informations that might be useful.

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
