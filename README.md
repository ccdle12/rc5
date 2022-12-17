# RC5 Cipher

Rivest describes the rc5 cipher here https://www.grc.com/r&d/rc5.pdf and includes a c reference implementation.

Further test cases can be found here https://datatracker.ietf.org/doc/html/draft-krovetz-rc6-rc5-vectors-00#section-4

## Docs

```
cargo doc --open
```

## Test

```
cargo test
```

## Fuzzing

Fuzzing requires:

* `llvm-symbolizer` - version 10
* [cargo fuzz](https://github.com/rust-fuzz/cargo-fuzz)

Run:

```
cargo fuzz run <name_of_fuzz_target>
```
