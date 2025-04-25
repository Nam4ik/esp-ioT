#!/bin/bash

rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

export IDF_PATH=/opt/esp-idf
source $IDF_PATH/export.sh
espup update
#export RUSTFLAGS="-C link-arg=-Tesp32c3/rom.ld -C link-arg=-Tesp32c3/sections.ld -C link-arg=-Tesp32c3/esp32c3.ld -C linker-plugin-lto=yes"
export ESP_IDF_PATH=/opt/esp-idf
export IDF_PATH=/opt/esp-idf

# Очистка и пересборка проекта
cargo clean
cargo update
cargo +nightly build -Z build-std=core,compiler_builtins
