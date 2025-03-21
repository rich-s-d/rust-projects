Rust Commands

```sh
rustc main.rs # using the rust compiler is fine for smaller projects
./main

cargo new hello_cargo # using cargo is better with multiple source files to coordinate the build, and for dependency management
cargo build # creates target in target/debug
./target/debug/hello_cargo # execute the target executable.
cargo run # build + execute. Does not watch for changes to a file, but https://crates.io/crates/cargo-watch does
cargo check # check that it compiles, does not build executable

cargo build --release # optimized for release, creates target in target/release

cargo doc --open # builds docs from the crates used in cargo.toml and displays them locally in your browser.

cargo new adder --lib # whenever a new library is created, a test module with a test function is automatically generated.

cargo test # picks up tests annotated with #[test] and runs them.
```