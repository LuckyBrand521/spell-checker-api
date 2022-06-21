# RESTful API With Rust


**RESTful Rust** is straightforward REST API example written in Rust. It shows how to _implement_ and _test_ GET, POST, PUT and DELETE methods with amazing [Warp](https://crates.io/crates/warp) web server framework.

### Getting started

To run the project locally:

1. install **rustup** by following the [instructions](https://www.rust-lang.org/tools/install)
2. add **clippy** (collection of lints) and **rustfmt** (code formatter) by running `rustup component add clippy` and `rustup component add rustfmt` accordingly
3. to **start an API** enter project's directory and run `cargo run`

### Dependencies overview

| Dependency                                                                                            | Description                                                    |
| ----------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| [warp](https://crates.io/crates/warp)                                                                 | Composable web server framework with powerful _filters_ system |
| [serde](https://crates.io/crates/serde)                                                               | Library for _serializing_ and _deserializing_ data structure                            |
| [log](https://crates.io/crates/log) + [pretty_env_logger](https://crates.io/crates/pretty_env_logger) | Simple logger (by default enabled in _debug_ mode)             |

