#!/bin/bash

cd "$(dirname "$0")"

arm-none-linux-gnueabihf-gdb -x gdb_run_args.txt --args ../target/thumbv7em-none-eabihf/debug/rusty_disco