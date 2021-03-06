# GBMU

Rust GameBoy Advanced Emulator

## Documentation

the documentation is available [here](#docs/Home.md)

## Dependencies

### For All

#### Rust configuration

This project require the rust toolchain: `1.60.X`, you can check it with the following command:

Outside of the toolchain version, we use `clippy` and `rustfmt`.
You can install it with the following command:

```sh
rustup component add clippy rustfmt
```

### For Linux

Building this project on linux requires `cmake`.

#### using apt

```sh
apt update && apt install cmake
```

#### using pacman

```sh
pacman -Sy cmake
```

#### using brew

```sh
brew install cmake
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

### Customizing the build

You can customize the build by activating certain feature by using the `--features` flags (comma separated)

| Name                         | Description                                           | Require                                                       |
| ---------------------------- | ----------------------------------------------------- | ------------------------------------------------------------- |
| fps                          | show fps of previous frames in the debugger           |                                                               |
| gb-lcd/debug_render          | allow the lcd's part to be debugged such as VRAM, OAM |                                                               |
| debug_render                 | change the UI to be able to launch lcd's debug tools   | `gb-lcd/debug_render`                                         |
| gb-cpu/registers_logs        | at each opcode log the state of the cpu register      |                                                               |
| gb-cpu/trace_jump            | enable trace for jump microcode                       |                                                               |
| gb-cpu/debug_decoded_opcode  | enable decoded opcode log                             |                                                               |
| registers_logs               | alias of `gb-cpu/registers_logs`                      |                                                               |
| gb-bus/trace_bus_full        | enable read/write trace for the bus                   | `gb-bus/trace_bus_write`, `gb-bus/trace_bus_read`             |
| gb-bus/trace_bus_write       | enable write trace for the bus                        |                                                               |
| gb-bus/trace_bus_read        | enable read trace for the bus                         |                                                               |
| gb-bus/trace_simple_rw_full  | enable read/write trace for simple container          | `gb-bus/trace_simple_rw_read`, `gb-bus/trace_simple_rw_write` |
| gb-bus/trace_simple_rw_read  | enable read trace for the simple container            |                                                               |
| gb-bus/trace_simple_rw_write | enable write trace for the simple container           |                                                               |
| gb-roms/debug_mbcs_register  | enable debug for debugging mbcs register edition      |                                                               |
