# rust-gitlab-ci

Utilities and images for Rust GitLab CI/CD pipelines.

## GitLab CI/CD Template

This repository contains a template for Rust crates/workspaces (`rust.gitlab-ci.yml`) that should cover most use cases.
It can be included like this:

```yaml
include: https://gitlab.com/TobiP64/rust-gitlab-ci/-/raw/master/rust.gitlab-ci.yml
```

## gitlab-report

A command line utility to generate GitLab compatible reports from cargo JSON output.

## Docker Images

### latest

A Fedora based image with the GNU toolchain.

Components:

- cargo-x86_64-unknown-linux-gnu
- clippy-x86_64-unknown-linux-gnu
- rust-std-wasm32-unknown-unknown
- rust-std-wasm32-wasi
- rust-std-x86_64-unknown-linux-gnu
- rust-std-x86_64-unknown-linux-musl
- rustc-x86_64-unknown-linux-gnu
- rustfmt-x86_64-unknown-linux-gnu

Tools:

- cargo-audit
- cargo-criterion
- cargo-expand
- cargo-geiger
- gitlab-report
- grcov
- wasm-bindgen-cli
- cargo-binutils
- cargo-cache

Other Packages:

- openssl-devel (required by cargo-audit)
- musl-gcc
- musl-devel
- findutils

### lld-musl

An Alpine based image, that uses LLD as the default linker and musl libc. This image does not contain the standard
library, that means the `-Z build-std` flag has to be set for builds. This image currently has various issues, thus
it is not recommended for production environments.

Components:

- cargo-x86_64-unknown-linux-musl
- clippy-x86_64-unknown-linux-musl
- llvm-tools-preview-x86_64-unknown-linux-musl
- rust-src
- rustc-x86_64-unknown-linux-musl
- rustfmt-x86_64-unknown-linux-musl

Tools:

- cargo-audit
- cargo-criterion
- cargo-expand
- cargo-geiger
- gitlab-report
- grcov
- wasm-bindgen-cli
- cargo-binutils
- cargo-cache
- Allure

Other Packages:

- musl-dev
- libgcc
- openssl (required by cargo-audit)
- curl
- clang
- openjdk11-jre-headless (required by Allure)