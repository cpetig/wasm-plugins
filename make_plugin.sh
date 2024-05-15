# #!/bin/sh
(cd app; cargo component build)
(cd plugin1; cargo component build)
wasm-tools compose target/wasm32-wasi/debug/app.wasm -d target/wasm32-wasi/debug/plugin1.wasm -o component1.wasm
wasmtime -S preview2 component1.wasm
# wac encode -o component2.wasm compose.wac --dep test:plugin=plugin1/target/wasm32-wasi/debug/plugin1.wasm --dep test:app=app/target/wasm32-wasi/debug/app.wasm

(cd app2; cargo component build)
(cd plugin3; cargo component build)
wasm-tools compose target/wasm32-wasi/debug/app2.wasm -d target/wasm32-wasi/debug/plugin1.wasm -o component3.wasm
wasm-tools compose component3.wasm -d target/wasm32-wasi/debug/plugin3.wasm -o component4.wasm
wasmtime -S preview2 component4.wasm

(cd plugin2; cargo component build)
(cd adapter; cargo component build)
wasm-tools compose target/wasm32-wasi/debug/adapter.wasm -d target/wasm32-wasi/debug/plugin2.wasm -o plugin2b.wasm
wasm-tools compose component3.wasm -d plugin2b.wasm -o component5.wasm
wasmtime -S preview2 component5.wasm
