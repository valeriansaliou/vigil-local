Packaging
=========

This file contains quick reminders and notes on how to package Vigil Local.

We consider here the packaging flow of Vigil Local version `1.0.0` for Linux.

1. **How to bump Vigil Local version before a release:**
    1. Bump version in `Cargo.toml` to `1.0.0`
    2. Execute `cargo update` to bump `Cargo.lock`

2. **How to update Vigil Local on Crates:**
    1. Publish package on Crates: `cargo publish --no-verify`

3. **How to build Vigil Local, package it and release it on GitHub (multiple architectures):**
    1. Install the cross-compilation utility: `cargo install cross`
    2. Release all binaries: `./scripts/release_binaries.sh --version=1.0.0`
    3. Publish all the built archives on the [releases](https://github.com/valeriansaliou/vigil-local/releases) page on GitHub

4. **How to update Docker image:**
    1. `docker build .`
    2. `docker tag [DOCKER_IMAGE_ID] valeriansaliou/vigil-local:v1.0.0` (insert the built image identifier)
    3. `docker push valeriansaliou/vigil-local:v1.0.0`
