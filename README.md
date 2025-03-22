# Measurement Logging Utility for the Sensirion SEN6x

This utility will continually take measurement readings from the Sensirion SEN6x series devices and emit them to various output destinations in JSON or CSV format.

Only Linux targets are currently supported. Raspbian on the Raspberry Pi 5 is the validation environment.

![example](./example.svg)

## Installation

### Installing from Source

Installation from source requires the [Rust toolchain](https://rust-lang.org).

#### Install Locally from Git Source Code

```sh
cargo install --git https://github.com/sbruton/sen6x-logger
```

### Cross-Compilation

Cross-compiling from source requires the [Rust toolchain](https://rust-lang.org).

#### Cross-Compile for Linux ARM targets from Ubuntu/Debian x86
```sh
# Install ARM build tooling
sudo apt-get install gcc-aarch64-linux-gnu libc6-dev-arm64-cross

# Install Linux ARM Rust target
rustup target add aarch64-unknown-linux-gnu

# Compile binary for Linux ARM
RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --release

# Resulting binary at `target/aarch64-unknown-linux-gnu/release/sen6x-logger`
```

#### Cross-Compile for Linux ARM targets from macOS
```sh
# Install ARM build tooling using homebrew
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-gnu

# Install Linux ARM Rust target
rustup target add aarch64-unknown-linux-gnu

# Compile binary for Linux ARM
cargo build --release

# Resulting binary at `target/aarch64-unknown-linux-gnu/release/sen6x-logger`
```

## Command Line Usage

```sh
# Print CLI usage information
sen6x-logger --help

# Log to STDOUT in JSON format
sen6x-logger /dev/i2c-1 \
    --stdout json

# Log to STDOUT in CSV format
sen6x-logger /dev/i2c-1 \
    --stdout csv

# Log to file in JSON format
sen6x-logger /dev/i2c-1 \
    --file output.json \
    --file-format json

# Log to file in CSV format
sen6x-logger /dev/i2c-1 \
    --file output.csv \
    --file-format csv

# Log to MQTT broker in JSON format
sen6x-logger /dev/i2c-1 \
    --mqtt-broker localhost:1883 \
    --mqtt-topic sen6x \
    --mqtt-format json

# Log to MQTT broker in CSV format
sen6x-logger /dev/i2c-1 \
    --mqtt-broker localhost:1883 \
    --mqtt-topic sen6x \
    --mqtt-format csv

# Log to multiple locations with mixed formats
sen6x-logger /dev/i2c-1 \
    --stdout csv \
    --file output.json \
    --file-format json \
    --mqtt-broker localhost:1883 \
    --mqtt-topic sen6x \
    --mqtt-format json
```