FROM alpine:latest AS build
ENV RUSTFLAGS="-Copt-level=z -Clinker=rust-lld -Clink-arg=-L/usr/lib -Clink-arg=-L/usr/lib/gcc/x86_64-alpine-linux-musl/10.3.1" \
	RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:$PATH
RUN apk add --no-cache libgcc gcc pkgconf libc-dev openssl-dev zlib-dev libssh2-dev curl-dev && \
	wget -qO- https://sh.rustup.rs > rustup-init.sh && \
	chmod +x ./rustup-init.sh && \
	./rustup-init.sh -y --verbose --profile minimal --no-modify-path && \
	cargo install gitlab-report cargo-audit cargo-binutils wasm-bindgen-cli

FROM alpine:latest
ARG ALLURE_VERSION="2.15.0"
ENV RUSTFLAGS="-Clinker=rust-lld"
	RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:/allure/bin/:$PATH
COPY --from=build /usr/local/cargo/bin /usr/local/cargo/bin
RUN apk add --no-cache libgcc openssl openjdk11-jre-headless && \
	wget -qO- https://sh.rustup.rs > rustup-init.sh && \
	chmod +x ./rustup-init.sh && \
	./rustup-init.sh --default-toolchain none --verbose -y && \
	rustup toolchain install stable beta nightly --profile minimal --component clippy --target x86_64-unknown-linux-musl mips64el-unknown-linux-muslabi64 aarch64-unknown-linux-musl wasm32-unknown-unknown wasm32-wasi && \
	rustup component add rustfmt llvm-tools-preview --toolchain stable && \
	rm -rf /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/bin/gcc-ld && \
    rm -rf /usr/lib/jvm/java-11-openjdk/legal && \
	wget -qO- https://repo.maven.apache.org/maven2/io/qameta/allure/allure-commandline/$ALLURE_VERSION/allure-commandline-$ALLURE_VERSION.tgz | tar -xz && \
	mv allure-$ALLURE_VERSION /allure