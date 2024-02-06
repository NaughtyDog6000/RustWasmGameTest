## BUILD / RUN

## Build Process

### web

cargo build --target wasm32-unknown-unknown --release

### native

cargo build --release

## Serving using npx serve

### linux (for pm2)

./serve.sh

### other

serve wasm_help -p 6964
