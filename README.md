# `cargo-gpu` & Miri

This repository shows various issues with the usage of `cargo-gpu` if it was called with Miri (by `cargo miri run` or `cargo miri test` commands)

Workspace of the repository contains multiple crates:

- `unsafe_abstraction` contains unsound code which is tested by Miri
- `simple_shader` compiles to SPIR-V by builder crates using `rust-gpu` project
- `simple_shader_builder_command` compiles `simple_shader` by using combination of local `cargo-gpu` installation and `std::process::Command`
- `simple_shader_builder_api` compiles `simple_shader` by using recently added `cargo-gpu` library API
