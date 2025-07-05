# love-wasm

This is an **EXPERIMENTAL** repository that tries to make the [LÖVE][] API available as [WASM components][].
That means games can be written in other languages, like Rust, C# or python, and still use the existing LÖVE codebase as a backend.

## Project structure

The directory `wit` contains the [wit][] definitions for the (supported parts of) the LÖVE API.

The directory `examples` contains example "games", that makes use of these definitions. See the included README's on how to build the examples, as the instructions vary by language.

The directory `host` contains a native (C++) application that uses [wasmtime][] to expose the LÖVE API to the webassembly code it's hosting.

[LÖVE]: https://love2d.org
[WASM components]: https://component-model.bytecodealliance.org/
[wit]: https://component-model.bytecodealliance.org/design/wit.html
[wasmtime]: https://wasmtime.dev/
