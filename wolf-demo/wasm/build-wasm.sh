#!/bin/sh

set -ex

# This example requires to *not* create ES modules, therefore we pass the flag
# `--target no-modules`
wasm-pack build --out-dir wasm/www/pkg --target web #no-modules