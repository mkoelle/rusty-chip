# RUSTY CHIP

## Setup

1. Run [rustup](https://rustup.rs/) \
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. Install [espup](https://github.com/esp-rs/espup) globally (a tool for installing and maintaining the required toolchains for Espressif) \
`cargo install espup --locked`
1. Install toolchains \
`espup install --targets=esp32,esp32s2,esp32s3`
    - add the esp tooling to your profile \
    `. $HOME/export-esp.sh`

`rustup +esp target add xtensa-esp32-espidf` \
`export CARGO_TARGET_DIR=target` \
`cargo +esp build --target xtensa-esp32-espidf`

https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/linux-macos-setup.html
https://docs.esp-rs.org/std-training/02_2_software.html

## Extra rust tooling

```sh
# Cargo-Outdated to look for out of date crates
cargo install cargo-outdated
# Can be run as `cargo outdated -R`

```