FROM rust:1.54-slim

RUN rustup component add rustfmt clippy \
  && sh -c "(set -x; cargo --version; rustc --version; cargo fmt --version; cargo clippy --version) > /etc/rust_toolchain_version 2>&1" \
  && apt-get update && apt-get install -y \
  cmake \
  libgtk-3-dev \
  curl \
  clang \
  && useradd tester

USER tester
COPY --chown=tester:tester . /app/
WORKDIR /app
RUN cargo build --workspace && cargo build --workspace --examples

# ENTRYPOINT [ "target/debug/gbmu" ]
