#!/bin/bash

cd "$(dirname "$0")"
TARGETS_DIR="../target/thumbv7em-none-eabihf/debug"

arm-none-linux-gnueabihf-objcopy -O binary ${TARGETS_DIR}/rusty_disco ${TARGETS_DIR}/rusty_disco.bin
st-flash write ${TARGETS_DIR}/rusty_disco.bin 0x8000000