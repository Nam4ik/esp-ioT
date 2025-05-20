#!/bin/bash

build_with_esp() {

    if ! command -v espup &> /dev/null; then
        echo "Error: espup is not installed. To use this method get espup and install it. (espup install)"
        exit 1
    fi

    read -p "Update espup before build? (y/n): " UPDATE_ESPUP
    if [[ $UPDATE_ESPUP == "y" ]]; then
        espup update
    fi

    export IDF_PATH=/opt/esp-idf
    source $IDF_PATH/export.sh
    export ESP_IDF_PATH=/opt/esp-idf
    export IDF_TOOLS_PATH=~/.espressif

    echo "Building by espup..."
    cargo clean
    cargo update
    rustup run esp cargo build --target xtensa-esp32-espidf -Z build-std=core,compiler_builtins
}


build_with_cargo() {
    echo "Building by cargo LLVM target... xtensa-esp32-espidf"
    cargo clean
    cargo update
    cargo +nightly build -Z build-std=core,alloc,compiler_builtins --target xtensa-esp32-espidf.json
}

build_with_cargo_elf(){
    echo "Building by cargo LLVM target... xtensa-esp32-none-elf"
    cargo clean
    cargo update
    cargo +nightly build -Z build-std=core,alloc,compiler_builtins --target xtensa-esp32-none-elf.json
}

echo "Choose build method:"
echo "1) Build by espup (espup package must be installed. )"
echo "2) Build by LLVM target - `xtensa-esp32-espidf.json`"
echo "3) Build by LLCM target - `xtensa-esp32-none-elf.json`"
echo "WARNING: Use second and third variations only if you cant get espup, build can broke or usupported target or targets conflict error.
read -p "Enter a number (1/3): " METHOD

case $METHOD in
    1)
        build_with_esp
        ;;
    2) 
       build_with_cargo
        ;;
    3) 
       build_with_cargo_elf
        ;;
    *)
        echo "Uncorrect choose. Enter 1-3 number."
        exit 1
        ;;
esac

echo "Build closed."
