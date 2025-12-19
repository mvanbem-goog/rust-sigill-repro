```
$ cargo clean && cargo run -Z build-std=std
[...]
Panicking locally

thread 'main' (149293) panicked at src/main.rs:4:5:
panic locally
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Panicking in core
Illegal instruction        (core dumped) cargo run -Z build-std=std
```
