FROM alpine:latest AS build
ENV RUSTFLAGS="-Copt-level=z -Clinker=rust-lld -Clink-arg=-L/usr/lib" \
	RUST_BACKTRACE=full \
	RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:$PATH
RUN apk add --no-cache libgcc clang pkgconf libc-dev openssl-dev zlib-dev libssh2-dev curl-dev && \
	wget -qO- https://sh.rustup.rs > rustup-init.sh && \
	chmod +x ./rustup-init.sh && \
	./rustup-init.sh -y --verbose  --no-modify-path --default-toolchain stable --profile minimal --component llvm-tools-preview && \
    ln -sf /usr/lib/libgcc_s.so.1 /usr/lib/libgcc_s.so && \
	ln -sf /usr/bin/clang /usr/bin/cc && \
    ln -sf /usr/local/rustup/toolchains/stable-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/bin/rust-lld /usr/bin/ld && \
	ln -sf /usr/local/rustup/toolchains/stable-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/bin/llvm-ar /usr/bin/ar && \
	ln -sf /usr/local/rustup/toolchains/stable-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/* /usr/lib/ && \
	cargo install --color=always gitlab-report grcov wasm-bindgen-cli cargo-audit cargo-binutils cargo-expand
#	cargo install --color=always gitlab-report grcov wasm-bindgen-cli cargo-audit cargo-geiger cargo-binutils cargo-expand

FROM alpine:latest
ARG PREBUILT_STD="true"
ARG RUST_TARGETS="x86_64-unknown-linux-musl mips64el-unknown-linux-muslabi64 aarch64-unknown-linux-musl wasm32-unknown-unknown wasm32-wasi"
ARG ALLURE_VERSION="2.15.0"
ENV RUSTFLAGS="-Clinker=rust-lld -Clink-arg=-L/usr/lib" \
	RUST_BACKTRACE=full \
	RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:/allure/bin/:$PATH
COPY --from=build /usr/local/cargo/bin /usr/local/cargo/bin
RUN apk add --no-cache musl-dev libgcc openssl curl clang openjdk11-jre-headless && \
	wget -qO- https://sh.rustup.rs > rustup-init.sh && \
	chmod +x ./rustup-init.sh && \
    { \
    	if [[ $PREBUILT_STD == "true" ]]; then \
    		./rustup-init.sh --verbose -y --default-toolchain none  --no-modify-path && \
    		rustup --verbose toolchain install stable beta nightly --profile minimal --component clippy --target $RUST_TARGETS && \
    		rustup --verbose component add --toolchain stable rustfmt llvm-tools-preview; \
    	else \
   			./rustup-init.sh --verbose -y --no-modify-path --default-toolchain nightly --profile minimal --component clippy rust-src rustfmt llvm-tools-preview && \
    		rustup --verbose component remove rust-std; \
    	fi \
    } && \
    ln -sf /usr/lib/libgcc_s.so.1 /usr/lib/libgcc_s.so && \
	ln -sf /usr/bin/clang /usr/bin/cc && \
    ln -sf /usr/local/rustup/toolchains/stable-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/bin/rust-lld /usr/bin/ld && \
    ln -sf /usr/local/rustup/toolchains/stable-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/bin/llvm-ar /usr/bin/ar && \
    ln -sf /usr/local/rustup/toolchains/stable-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/* /usr/lib/ && \
	rm -rf /usr/local/rustup/toolchains/*-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/bin/gcc-ld && \
    rm -rf /usr/lib/jvm/java-11-openjdk/legal && \
	wget -qO- https://repo.maven.apache.org/maven2/io/qameta/allure/allure-commandline/$ALLURE_VERSION/allure-commandline-$ALLURE_VERSION.tgz | tar -xz && \
	mv allure-$ALLURE_VERSION /allure