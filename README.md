# GBMU

Rust GameBoy Advanced Emulator

## Documentation

the documentation is available [here](#docs/Home.md)

## Dependencies

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
