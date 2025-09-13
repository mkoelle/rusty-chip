# RUSTY CHIP

## Setup

### MAC

1. Run [rustup](https://rustup.rs/) \
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. Install [espup](https://github.com/esp-rs/espup) globally (a tool for installing and maintaining the required toolchains for Espressif) \
   `. "$HOME/.cargo/env"`
   `find and delete the rust-toolchain.toml file`
   `cargo install espup`
1. Install toolchains \
   `espup install` \
   `espup install --targets=esp32,esp32s2,esp32s3` - add the esp tooling to your profile \
    `. $HOME/export-esp.sh`
1. If using brew
   `brew install certifi`
   `curl -k -o .embuild/espressif/espidf.constraints.v5.3.txt https://dl.espressif.com/dl/esp-idf/espidf.constraints.v5.3.txt`
   `cargo install ldproxy`

---

`rustup +esp target add xtensa-esp32-espidf` \
`export CARGO_TARGET_DIR=target` \
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

## Extra rust tooling

```sh
# Cargo-Outdated to look for out of date crates
cargo install cargo-outdated
# Can be run as `cargo outdated -R`

```

## troubleshooting

### cant find crate for 'core'

https://pfesenmeier.github.io/wsl2-and-embedded-development/
https://www.instructables.com/ESP32-ESP32C3-Blink-Test-Rust-Development-in-Windo/
https://developer.mamezou-tech.com/en/blogs/2025/05/19/using-rust-02/
