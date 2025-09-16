# RUSTY CHIP

A collection of ESP32 projects written in rust for Robot and IoT club.

## Repository Layout

Individual solutions are kept under the projects folder.

Where possible boards are simulated with wokwi.
These are the `diagram.json` and `wokwi.toml` files found in each of the projects
A free license is sufficient for running and creating pinouts.
Creating a diagram can be done at [wokwi.com/projects/new/esp32](https://wokwi.com/projects/new/esp32), then copied into the project folder.

Github actions and vscode folders are included for quality checks and quality of life help.

## Setup

Getting the first build to work when new to rust and the esp32 is tricky.
The instructions below are as close to complete as was recalled.

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



python3 -m pip install --upgrade certifi

mkdir -p .embuild/espressif

curl -k -o .embuild/espressif/espidf.constraints.v5.3.txt https://dl.espressif.com/dl/esp-idf/espidf.constraints.v5.3.txt

cargo install cargo-generate
cargo install espup
cargo install espflash
cargo install cargo-espflash

cargo build --target xtensa-esp32-espidf

## Extra rust tools

```sh
# Cargo-Outdated to look for out of date crates
cargo install cargo-outdated
# Can be run as `cargo outdated -R`
```

## References

- https://docs.esp-rs.org/std-training
- https://jamesmcm.github.io/blog/beginner-rust-esp32-lcdsnake/

Guides for getting started with ESP and Rust in WSL on windows

- https://pfesenmeier.github.io/wsl2-and-embedded-development/
- https://www.instructables.com/ESP32-ESP32C3-Blink-Test-Rust-Development-in-Windo/
- https://developer.mamezou-tech.com/en/blogs/2025/05/19/using-rust-02/

Guides for getting started with ESP and Rust on MAC

- https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/linux-macos-setup.html