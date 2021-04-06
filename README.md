## Ruszzer
A Fuzzer implementation in Rust following the guidelines exposed in https://www.fuzzingbook.org/.

### Running the project

1. Install `rustc` and `cargo` (it can be done using the [rustup](https://www.rust-lang.org/tools/install) script);
2. Compile your fuzzy target using `gcc` and the `--coverage` flag:
```sh
$ cd fuzzy_targets
$ gcc --coverage -o cgi_decode cgi_decode.c
$ cd ..
```

3. Build...
```
$ cargo update
$ cargo build
```

4. .. and fuzz :)
```sh
# Check for required and available parameters
$ cargo run -- --help
# Example: Fuzz the "cgi_decode" target choosing a "boosted_greybox" strategy running 200 trials and using "http://google.com/search?q=foo" as the initial seed
$ cargo run -- -i fuzzy_targets/cgi_decode -f boosted_greybox -t 200 -s "http://google.com/search?q=foo"
```