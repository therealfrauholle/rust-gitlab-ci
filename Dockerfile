FROM fedora:34
ARG RUST_TARGETS="x86_64-unknown-linux-musl wasm32-wasi wasm32-unknown-unknown"
ENV RUST_BACKTRACE=full \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/allure/bin/:$PATH
RUN dnf install -y openssl-devel musl-gcc musl-devel findutils && \
    dnf clean all && \
    curl https://sh.rustup.rs > rustup-init.sh && \
    chmod +x ./rustup-init.sh && \
    ./rustup-init.sh --verbose -y --default-toolchain none  --no-modify-path && \
    rustup --verbose toolchain install stable beta nightly --profile minimal --component clippy --target $RUST_TARGETS && \
    rustup --verbose component add --toolchain stable rustfmt && \
    cargo install --color=always cargo-audit cargo-criterion cargo-expand cargo-geiger gitlab-report grcov wasm-bindgen-cli cargo-binutils cargo-cache && \
    cargo cache -a