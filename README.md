# rotary-encoder

Rust library for decoding the outputs of rotary encoders in conjunction with
[futures](https://github.com/alexcrichton/futures-rs). You will likely want to
use this with [sysfs_gpio](https://github.com/rust-embedded/rust-sysfs-gpio)
and [tokio-core](https://github.com/tokio-rs/tokio-core) (see
`examples/echo.rs`).
