
prepare-apple:
	rustup toolchain install nightly
	rustup target add aarch64-apple-ios-sim --toolchain nightly && rustup target add aarch64-apple-ios x86_64-apple-ios
	rustup component add rust-src --toolchain nightly
	make install-uniffi-bindgen

prepare-android:
	rustup toolchain install stable
	rustup target add x86_64-linux-android
	rustup target add x86_64-unknown-linux-gnu
	rustup target add aarch64-linux-android
	rustup target add armv7-linux-androideabi
	rustup target add i686-linux-android
	make install-uniffi-bindgen

install-uniffi-bindgen:
	cargo install uniffi_bindgen --version 0.17.0