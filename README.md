# README

Reproduction for missing IntelliSense for cargo-component with Rust Analyzer.



## Usage

1. Open in VSCode with active Rust Analyzer extension.
1. Confirm bindings are generated in [target/bindings/foo/bindings.rs](target/bindings/foo/bindings.rs).
1. Confirm no IntelliSense when cursor is placed over `api::Guest` trait in [foo/src/lib.rs](foo/src/lib.rs).
1. Confirm running command "Expand macro recursively at caret" gives empty `bindings` when cursor is placed over `cargo_component_bindings::generate!()` macro in [foo/src/lib.rs](foo/src/lib.rs).
