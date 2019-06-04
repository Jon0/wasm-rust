#!/bin/sh

set -ex

wasm-pack build --target web
# wasm-pack build --target no-modules

python3 -m http.server

