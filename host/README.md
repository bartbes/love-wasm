# Host

Implements a host application that uses wasmtime to host webassembly applications, and provides the LÖVE API to them.

## Prerequisites

Probably requires linux, cmake, a modern c++ compiler, a recent rust build environment and the LÖVE build dependencies should be available.
The build automatically pulls in LÖVE itself, and wasmtime and its dependencies.

## Build instructions

There is a `CMakePresets.json` file that should get you set up. Use it as follows:

```sh
cmake --preset dev
```

This should be equivalent to the following configure command:

```sh
cmake -S. -Bbuild -GNinja -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
```

**Note:** The first configure takes a while as it downloads the dependencies. It may look like cmake is stuck, but give it a couple of minutes.

Building is then as simple as:

```sh
cmake --build build
```

This should create the binary in `build/host`.
