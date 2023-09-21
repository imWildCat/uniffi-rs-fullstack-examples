# Fullstack Examples of UniFFI-rs

Full documentation of the UniFFI crates can be found [here](https://mozilla.github.io/uniffi-rs/Overview.html).

Another demo including building Rust for WASM (WebAssembly): <https://github.com/imWildCat/rust-mobile-web-demo>

If you find this project useful, please consider support me: <https://github.com/sponsors/imWildCat>.
Thanks!

## Questions?

<https://github.com/imWildCat/uniffi-rs-fullstack-examples/discussions>

## Prerequisites

### Rust

Please read <https://www.rust-lang.org/tools/install>.

### Android

1. JDK 11.x, Android SDK and NDK, Android Studio is optional. For more details, please visit [.github/workflows/build.yml](.github/workflows/build.yml).
2. Install Rust toolchains for Android: `make prepare-android`

### iOS

1. Latest Xcode (14.3)
1. Rust toolchains for iOS: `make prepare-apple` or `rustup target add aarch64-apple-ios-sim --toolchain nightly && rustup target add aarch64-apple-ios x86_64-apple-ios`. Check installaion:
    ```shell
    $ rustup target list --installed | grep ios
    aarch64-apple-ios
    aarch64-apple-ios-sim
    x86_64-apple-ios
    ```
    Please note that `aarch64-apple-ios-sim` requries the nightly toolchain (<https://doc.rust-lang.org/nightly/rustc/platform-support/aarch64-apple-ios-sim.html>).
1. Install `uniffi-bindgen`: `cargo install uniffi_bindgen`, details: <https://mozilla.github.io/uniffi-rs/tutorial/Prerequisites.html>

## Get Started

### Hello

Please read [hello/README.md](hello/README.md).

## License

MIT
