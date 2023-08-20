STATIC_LIB_NAME := libhello.a
TARGET_DIR := ../target

apple:
	@make build-targets
	@make bindgen-swift
	@make assemble-frameworks
	@make xcframeowrk
	@make cp-xcframeowrk-source

build-targets:
	cargo build --lib --release --target x86_64-apple-ios
	cargo build --lib --release --target aarch64-apple-ios-sim
	cargo +nightly build -Z build-std --lib --release --target aarch64-apple-ios-macabi
	cargo +nightly build -Z build-std --lib --release --target x86_64-apple-ios-macabi
	cargo build --lib --release --target aarch64-apple-ios

bindgen-swift:
	cargo run -p uniffi-bindgen generate --language swift --lib-file $(TARGET_DIR)/aarch64-apple-ios/release/$(STATIC_LIB_NAME) src/hello.udl
	sed -i '' 's/module\ HelloFFI/framework\ module\ HelloFFI/' src/HelloFFI.modulemap

bindgen-kotlin:
	cargo run -p uniffi-bindgen generate src/hello.udl --language kotlin -o platforms/android/UniffiRustExample/app/src/main/java
	sed -i '' 's/return "uniffi_Hello"/return "hello"/' platforms/android/UniffiRustExample/app/src/main/java/uniffi/Hello/Hello.kt

assemble-frameworks:
	cd $(TARGET_DIR) && find . -type d -name HelloFFI.framework -exec rm -rf {} \; || echo "rm failed"
	cd $(TARGET_DIR)/x86_64-apple-ios/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && pwd && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/aarch64-apple-ios-sim/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/aarch64-apple-ios/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/aarch64-apple-ios-macabi/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/x86_64-apple-ios-macabi/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources

xcframeowrk:
	lipo -create $(TARGET_DIR)/x86_64-apple-ios/release/HelloFFI.framework/HelloFFI $(TARGET_DIR)/aarch64-apple-ios-sim/release/HelloFFI.framework/HelloFFI -output $(TARGET_DIR)/aarch64-apple-ios-sim/release/HelloFFI.framework/HelloFFI
	lipo -create $(TARGET_DIR)/x86_64-apple-ios-macabi/release/HelloFFI.framework/HelloFFI $(TARGET_DIR)/aarch64-apple-ios-macabi/release/HelloFFI.framework/HelloFFI -output $(TARGET_DIR)/aarch64-apple-ios-macabi/release/HelloFFI.framework/HelloFFI
	rm -rf $(TARGET_DIR)/HelloFFI.xcframework || echo "skip removing"
	xcodebuild -create-xcframework -framework $(TARGET_DIR)/aarch64-apple-ios/release/HelloFFI.framework -framework $(TARGET_DIR)/aarch64-apple-ios-sim/release/HelloFFI.framework -framework $(TARGET_DIR)/aarch64-apple-ios-macabi/release/HelloFFI.framework -output $(TARGET_DIR)/HelloFFI.xcframework

cp-xcframeowrk-source:
	cp -r $(TARGET_DIR)/HelloFFI.xcframework platforms/apple/Hello/Sources
	cp src/Hello.swift platforms/apple/Hello/Sources/Hello

android:
	cd platforms/android/UniffiRustExample && ./gradlew cargoBuild --info
