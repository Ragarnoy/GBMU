FROM rust:1.54-slim

RUN rustup component add rustfmt clippy \
  && sh -c "(set -x; cargo --version; rustc --version; cargo fmt --version; cargo clippy --version) > /etc/rust_toolchain_version 2>&1" \
  && apt-get update && apt-get install -y \
  cmake \
  libgtk-3-dev \
  curl \
  clang \
  dbus-x11 \
  && useradd tester -d /home/tester -m

USER tester

COPY --chown=tester:tester Cargo.toml Cargo.lock /app/
COPY --chown=tester:tester gb-cpu/ /app/gb-cpu
COPY --chown=tester:tester gb-lcd/ /app/gb-lcd/
COPY --chown=tester:tester gb-ppu/ /app/gb-ppu/
COPY --chown=tester:tester gb-roms/ /app/gb-roms/
COPY --chown=tester:tester gb-bus/ /app/gb-bus/
COPY --chown=tester:tester gb-dbg/ /app/gb-dbg/
COPY --chown=tester:tester src/ /app/src/

WORKDIR /app
RUN cargo build --workspace && cargo build --workspace --examples

ENTRYPOINT [ "target/debug/gbmu" ]
