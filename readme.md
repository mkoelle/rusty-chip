# RUSTY CHIP

A collection of ESP32 projects written in rust for Robot and IoT club.

## Repository Layout

Individual solutions are kept under the projects folder.

Where possible boards are simulated with wokwi.
These are the `diagram.json` and `wokwi.toml` files found in each of the projects
A free license is sufficient for running and creating pinouts.
Creating a diagram can be done at [wokwi.com/projects/new/esp32](https://wokwi.com/projects/new/esp32), then copied into the project folder.

Github actions and vscode folders are included for quality checks and quality of life help.

## Development

```sh
# build the project
cargo build
```

### Flashing the hardware

```sh
cargo espflash flash --target xtensa-esp32-espidf --monitor
```

## Setup

Getting the first build to work when new to rust and the esp32 is tricky.
The instructions below are as close to complete as was recalled.

### MAC

1. Run [rustup](https://rustup.rs/) \
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. Install [espup](https://github.com/esp-rs/espup) globally (a tool for installing and maintaining the required toolchains for Espressif) \
   `. "$HOME/.cargo/env"` \
   `cargo install espup --locked`
1. Install toolchains \
   `espup install`
   - add the esp tooling to your profile \
     `. $HOME/export-esp.sh`
1. Install ldproxy \
   `cargo install ldproxy`
1. If behind a proxy
   - install certifi \
     `brew install certifi`
   - Per project acquire espidf.constraints \
      `cd project/foo` \
      `mkdir -p .embuild/espressif` \
      `curl -k -o .embuild/espressif/espidf.constraints.v5.3.txt https://dl.espressif.com/dl/esp-idf/espidf.constraints.v5.3.txt`

Additional Guides for getting started with ESP and Rust on MAC

- [Espressif guide to getting started with linux and macos setup](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/linux-macos-setup.html)

### Windows

For windows you will need to setup WSL instead of working on the repo directly.
Rust does not yet play nice with [windows half support](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=registry)
for [long path names](https://github.com/rust-lang/cargo/issues/9770).
It is recommended to use ubuntu for the wsl distro.

1. install linux build tools
   - `sudo apt update` \
      `sudo apt install -y libssl-dev pkg-config curl build-essential gcc libudev-dev python3-venv`
1. install rust
   - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - ` . "$HOME/.cargo/env"` sourcing the cargo env should be automatic after restarting your session.
1. install [espup](https://github.com/esp-rs/espup)
   - `cargo install espup --locked`
1. install esp targets from espup
   - `espup install --targets esp32`
1. install remaining esp tooling
   - `cargo install espflash` \
      `cargo install ldproxy` \
      `cargo install cargo-generate` \
      `cargo install cargo-espflash`

To build projects first source the esp exports, then build.

```sh
. $HOME/export-esp.sh
cargo build
```

Additional Guides for getting started with ESP and Rust in WSL on windows

- https://pfesenmeier.github.io/wsl2-and-embedded-development/
- https://www.instructables.com/ESP32-ESP32C3-Blink-Test-Rust-Development-in-Windo/
- https://developer.mamezou-tech.com/en/blogs/2025/05/19/using-rust-02/

## Extra rust tools

```sh
# Cargo-Outdated to look for out of date crates
cargo install cargo-outdated
# Can be run after install as
cargo outdated -R
```

## References

- https://docs.esp-rs.org/std-training
- https://jamesmcm.github.io/blog/beginner-rust-esp32-lcdsnake/
