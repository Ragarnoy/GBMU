# GBMU

Rust GameBoy Advanced Emulator

## Documentation

the documentation is available [here](#docs/Home.md)

## Dependencies

### For All

#### Rust configuration

This project require the rust toolchain: `1.56.X`, you can check it with the following command:

```sh
$ cargo --version
cargo 1.56.0 (4ed5d137b 2021-10-04)
$ rustc --version
rustc 1.56.0 (09c42c458 2021-10-18)
```

Outside of the toolchain version, we use `clippy` and `rustfmt`.
You can install it with the following command:

```sh
rustup component add clippy rustfmt
```

### For Linux

Building this project on linux requires `cmake` and `gtk3`.

#### using apt

```sh
apt update && apt install cmake libgtk-3-dev
```

#### using pacman

```sh
pacman -Sy cmake gtk3
```

#### using brew

```sh
brew install cmake gtk+3
```

### For MacOS

Building this project on MacOs requires `cmake`.

```sh
brew install cmake
```

## Building

Before building, you need to install the `bios` files that here included in the package

```sh
make bios
```

PS: if you want to also download the ROMS `make requirement`

After the `bios` files are installed, you can build using `cargo`

```sh
cargo build --workspace --examples
```
