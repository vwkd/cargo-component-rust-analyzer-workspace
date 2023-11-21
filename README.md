# README

Reproduction for missing IntelliSense for cargo-component `0.4.1` with Rust Analyzer. See [cargo-component#139](https://github.com/bytecodealliance/cargo-component/issues/139#issuecomment-1820609274).



## Requirements

- VSCode `1.84.2`
- Rust Analyzer extension `v0.3.1740`
- `cargo-component` package `0.4.1`


## Usage

1. Open repo in VSCode.
1. Confirm bindings are generated in [target/bindings/foo/bindings.rs](target/bindings/foo/bindings.rs#L6) automatically by Rust Analyzer.
1. Confirm no IntelliSense when cursor is placed over `api::Guest` trait in [foo/src/lib.rs#L6](foo/src/lib.rs#L6).
1. Confirm running command "rust analyzer: Expand macro recursively at caret" gives empty `bindings` when cursor is placed over `cargo_component_bindings::generate!()` macro in [foo/src/lib.rs#L1](foo/src/lib.rs#L1).
