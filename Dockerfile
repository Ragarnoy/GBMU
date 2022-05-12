FROM rust:1.60-slim

RUN rustup component add rustfmt clippy \
  && sh -c "(set -x; cargo --version; rustc --version; cargo fmt --version; cargo clippy --version) > /etc/rust_toolchain_version 2>&1" \
  && apt-get update && apt-get install -y \
  cmake \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libgtk-3-dev \
  curl \
  clang \
  dbus-x11 \
  && useradd tester -d /app -m

USER tester
COPY --chown=tester:tester assets/ /app/assets

COPY --chown=tester:tester Cargo.toml Cargo.lock /app/
COPY --chown=tester:tester gb-bus/ /app/gb-bus/
COPY --chown=tester:tester gb-clock/ /app/gb-clock/
COPY --chown=tester:tester gb-cpu/ /app/gb-cpu/
COPY --chown=tester:tester gb-apu/ /app/gb-apu/
COPY --chown=tester:tester gb-test/ /app/gb-test/
COPY --chown=tester:tester gb-breakpoint/ /app/gb-breakpoint/
COPY --chown=tester:tester gb-dbg/ /app/gb-dbg/
COPY --chown=tester:tester gb-dma/ /app/gb-dma/
COPY --chown=tester:tester gb-joypad/ /app/gb-joypad/
COPY --chown=tester:tester gb-lcd/ /app/gb-lcd/
COPY --chown=tester:tester gb-ppu/ /app/gb-ppu/
COPY --chown=tester:tester gb-roms/ /app/gb-roms/
COPY --chown=tester:tester gb-rtc/ /app/gb-rtc/
COPY --chown=tester:tester gb-timer/ /app/gb-timer/
COPY --chown=tester:tester src/ /app/src/

WORKDIR /app
RUN cargo build --release

ENTRYPOINT [ "target/release/gbmu" ]
