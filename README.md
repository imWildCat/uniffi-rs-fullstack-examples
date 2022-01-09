# Fullstack Examples of UniFFI-rs

Full documentation of the UniFFI crates can be found [here](https://mozilla.github.io/uniffi-rs/Overview.html).

Working in progress

## Prerequisites

### iOS

1. Latest Xcode (13.x)
1. Rust toolchains for iOS: `rustup target add aarch64-apple-ios-sim --toolchain nightly && rustup target add aarch64-apple-ios x86_64-apple-ios`. Check installaion:
    ```shell
    $ rustup target list --installed | grep ios
    aarch64-apple-ios
    aarch64-apple-ios-sim
    x86_64-apple-ios
    ```
    Please note that `aarch64-apple-ios-sim` requries the nightly toolchain (<https://doc.rust-lang.org/nightly/rustc/platform-support/aarch64-apple-ios-sim.html>).
1. Install `uniffi-bindgen`: `cargo install uniffi_bindgen`, details: <https://mozilla.github.io/uniffi-rs/tutorial/Prerequisites.html>

## License

MIT
