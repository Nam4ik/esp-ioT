#!/bin/bash

if [[ "$1" == "-nointeractive" ]]; then
    if [ ! -f "conf.env" ]; then
        echo "Ошибка: файл conf.env не найден."
        exit 1
    fi
    source conf.env
    case "$build_vendor" in
        "esp") METHOD=1 
        ;;
        
        "cargo-espdif") METHOD=2 
        ;;
        
        "cargo-none-elf") METHOD=3 
        ;;
        
        "cargo-riscv") METHOD=5 
        ;;
        
        "cargo-espup") METHOD=4 
        ;;
        
        *)
            echo "Ошибка: неверное значение build_vendor в conf.env"
            exit 1
            ;;
    esac
else

echo "Choose build method:"
echo "TARGET: Xtensa32:"
echo "1) Build by espup (espup package must be installed. )"
echo "2) Build by LLVM target - `xtensa-esp32-espidf.json`"
echo "3) Build by LLVM target - `xtensa-esp32-none-elf.json`"
echo "WARNING: Use second and third variations only if you cant get espup, build can broke or usupported target or targets conflict error."
echo "TARGET: RISCV32"
echo "4) Build by espup (espup package must be installed. )"
echo "5) Build by default cargo target riscv32imc-none-elf (unable to install target before run build)"
read -p "Enter a number (1/5): " METHOD

fi

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
    rustup run esp cargo build -Z build-std=core,alloc,compiler_builtins --release
   # --target xtensa-esp32-espidf
}

build_with_riscv_espup() {

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
    rustup run esp cargo build -Z build-std=core,alloc,compiler_builtins --target riscv32imac-esp-espidf --release
}

build_with_riscv_cargo(){
    echo "Building by cargo LLVM target... riscv32imc-none-elf"
    cargo clean
    cargo update
    cargo +nightly build -Z build-std=core,alloc,compiler_builtins --target riscv32imc-none-elf --release
}

build_with_cargo() {
    echo "Building by cargo LLVM target... xtensa-esp32-espidf"
    cargo clean
    cargo update
    cargo +nightly build -Z build-std=core,alloc,compiler_builtins --target xtensa-esp32-espidf.json --release
}

build_with_cargo_elf(){
    echo "Building by cargo LLVM target... xtensa-esp32-none-elf"
    cargo clean
    cargo update
    cargo +nightly build -Z build-std=core,alloc,compiler_builtins --target xtensa-esp32-none-elf.json --release
}

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
        
    4) build_with_riscv_espup
        ;;
        
    5) build_with_riscv_cargo
        ;;
        
    *)
        echo "Uncorrect choose. Enter 1-3 number."
        exit 1
        ;;
esac

echo "Build closed."
