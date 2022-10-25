


# Test and Coverage
Further explanation in the [Mozilla grcov website](https://github.com/mozilla/grcov)

```sh
rustup component add llvm-tools-preview
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

# Documentation
```sh
cargo doc --open
```