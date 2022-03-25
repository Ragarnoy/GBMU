fn main() {
    #[cfg(windows)]
    println!("cargo:rustc-link-arg-bins=-Wl,-zstack-size=16777216");
}
