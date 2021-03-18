## Ruszzer
A Fuzzer implementation in Rust following the guidelines exposed in https://www.fuzzingbook.org/.

### Running the project

1. Install `rustc` and `cargo` (it can be done using the [rustup](https://www.rust-lang.org/tools/install) script);
2. Compile your fuzzy target using `gcc` and the `--coverage` flag:
```sh
cd fuzzy_targets
gcc --coverage -o cgi_decode cgi_decode.c
cd ..
```
3. Manualy point to your target in the `src/main.rs` file (A WIP is in progress for a CLI):
```rust
let cgi_decode_program_runner = GCovBinaryRunner {
    binary_path: String::from("fuzzy_targets"),
    binary_name: String::from("cgi_decode"),
};
```

4. Build and run :)
```
cargo update
cargo build
cargo run
```
