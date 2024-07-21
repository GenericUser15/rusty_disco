#!/bin/bash

cd "$(dirname "$0")"

st-flash write ../target/thumbv7em-none-eabihf/debug/rusty_disco 0x8000000