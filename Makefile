
prepare-apple:
	rustup toolchain install nightly
	rustup target add aarch64-apple-ios-sim --toolchain nightly && rustup target add aarch64-apple-ios x86_64-apple-ios
	rustup component add rust-src --toolchain nightly
	cargo install uniffi_bindgen
