# Build Process

## web

RUN:
./webglbuild.bat
(runs cargo build release for wasm and then wasm build stuff)
(runs serve in the built directory so you can see if it works)
## native

cargo run --release
