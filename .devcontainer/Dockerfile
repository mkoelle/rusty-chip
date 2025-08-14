# Base: ESP-IDF v5.1 image
FROM espressif/idf:release-v5.1

# Install required build dependencies
RUN apt-get update && apt-get install -y \
    git curl xz-utils python3 python3-pip cmake ninja-build pkg-config libudev-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust (Espressif fork) + Xtensa toolchain using espup
ENV CARGO_HOME=/root/.cargo
ENV RUSTUP_HOME=/root/.rustup
ENV PATH=$CARGO_HOME/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
RUN cargo install espup --locked
RUN espup install --default-host x86_64-unknown-linux-gnu

# Ensure ESP environment is loaded for all shells
RUN echo '. $HOME/export-esp.sh' >> /root/.bashrc

# Install cargo-espflash (for flashing to real ESP32 boards)
RUN cargo install cargo-espflash

# Set workspace
WORKDIR /workspace
