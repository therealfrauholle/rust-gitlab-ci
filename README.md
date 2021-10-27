# rust-gitlab-ci

Utilities and images for Rust GitLab CI/CD pipelines.

## GitLab CI/CD Template

This repository contains a template for Rust crates/workspaces (`rust.gitlab-ci.yml`) and
It can be included like this:

```yaml
include: https://gitlab.com/TobiP64/rust-gitlab-ci/-/raw/master/rust.gitlab-ci.yml
```

## gitlab-report

A command line utility to generate GitLab compatible reports from cargo JSON output.

## Docker Image

This repository provides an alpine based docker image. It is configured to
use rust-lld and musl libc.

Components:

- llvm-tools-preview-x86_64-unknown-linux-musl (stable)
- rustfmt-x86_64-unknown-linux-gnu (stable)
- cargo-x86_64-unknown-linux-musl (stable)
- clippy-x86_64-unknown-linux-musl (stable)
- rustc-x86_64-unknown-linux-musl (stable)
- rust-std-x86_64-unknown-linux-musl (stable)
- rust-std-mips64el-unknown-linux-muslabi64 (stable)
- rust-std-aarch64-unknown-linux-musl (stable)
- rust-std-wasm32-unknown-unknown (stable)
- rust-std-wasm32-wasi (stable)
- cargo-x86_64-unknown-linux-musl (beta)
- clippy-x86_64-unknown-linux-musl (beta)
- rustc-x86_64-unknown-linux-musl (beta)
- rust-std-x86_64-unknown-linux-musl (beta)
- rust-std-mips64el-unknown-linux-muslabi64 (beta)
- rust-std-aarch64-unknown-linux-musl (beta)
- rust-std-wasm32-unknown-unknown (beta)
- rust-std-wasm32-wasi (beta)
- cargo-x86_64-unknown-linux-musl (nightly)
- clippy-x86_64-unknown-linux-musl (nightly)
- rustc-x86_64-unknown-linux-musl (nightly)
- rust-std-x86_64-unknown-linux-musl (nightly)
- rust-std-mips64el-unknown-linux-muslabi64 (nightly)
- rust-std-aarch64-unknown-linux-musl (nightly)
- rust-std-wasm32-unknown-unknown (nightly)
- rust-std-wasm32-wasi (nightly)

Tools:

- gitlab-report
- cargo-audit
- cargo-binutils
- wasm-bindgen-cli
- Allure

Other Packages:

- openssl (required by cargo-audit)
- openjdk11-jre-headless (required by Allure)

### build-std

There is also a very minimalistic Dockerfile that does not contain pre-compiled stdlib
and builds with `-Zbuild-std`. A docker image is not built in the CI pipeline, but is
supposed to be built manually.