# Perf

How can you evaluate the perfomance of `GBMU`

## Pre requisite

You need to have the executable `GBMU` builded (with the profile that you want to check the performance of) and available on your filesystem

### For development build

```sh
cargo build
ls target/debug/gbmu
```

### For release build

```sh
cargo build --release
ls target/release/gbmu
```

## Using Perf

```sh
perf record --call-graph dwarph target/(debug/release)/gbmu
perf script perf.data > perf.scipt
```

the you can upload the `perf.scipt` to [profiler.firefox](https://profiler.firefox.com)

## Sources

- [Performance profiling on Linux - Rust](https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html)
