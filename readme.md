# RUSTY CHIP

## Setup

`rustup`
`cargo add espup`
`cargo install espup --locked`
`espup install --targets=esp32,esp32s2,esp32s3`
`rustup +esp target add xtensa-esp32-espidf`
`export CARGO_TARGET_DIR=target`
`cargo +esp build --target xtensa-esp32-espidf`

https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/linux-macos-setup.html
https://docs.esp-rs.org/std-training/02_2_software.html

python3 -m pip install --upgrade certifi

mkdir -p .embuild/espressif

curl -k -o .embuild/espressif/espidf.constraints.v5.3.txt https://dl.espressif.com/dl/esp-idf/espidf.constraints.v5.3.txt

cargo install cargo-generate
cargo install espup
cargo install espflash
cargo install cargo-espflash

cargo build --target xtensa-esp32-espidf
