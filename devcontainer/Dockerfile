# .devcontainer/Dockerfile
FROM espressif/idf:release-v5.1

# Install Rust + xtensa target
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
 && . $HOME/.cargo/env \
 && rustup toolchain install nightly-2025-05-01 \
 && rustup default nightly-2025-05-01 \
 && rustup target add xtensa-esp32-none-elf

# Install cargo-espflash
RUN . $HOME/.cargo/env \
 && cargo install cargo-espflash

# Create workspace dir
WORKDIR /workspace