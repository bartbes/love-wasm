# Pong

This is a simple pong example. As it is just a proof that the wrappers work I haven't bothered to make it a good pong implementation. I'm also no expert at rust, so I'm sure the code isn't idiomatic.

## Build instructions

Make sure you have the `wasm32-wasip2` target installed. You can do this using `rustup` as follows:

```sh
rustup target add wasm32-wasip2
```

Building is then as simple as `cargo build` with the right target:

```sh
cargo build --target wasm32-wasip2
```

This will create a wasm file in `target/wasm32-wasip2/debug` which can then be loaded into the host application.
