STATIC_LIB_NAME := libhello.a
TARGET_DIR := ../target

apple:
	@make build-targets
	@make bindgen-swift
	@make assemble-frameworks
	@make xcframeowrk
	@make cp-xcframeowrk-source

build-targets:
	cargo build --lib --target x86_64-apple-ios
	cargo build --lib --target aarch64-apple-ios-sim
	cargo +nightly build -Z build-std --lib --target aarch64-apple-ios-macabi
	cargo +nightly build -Z build-std --lib --target x86_64-apple-ios-macabi
	cargo build --lib --target aarch64-apple-ios
	cargo build --lib --target aarch64-apple-darwin 
	cargo build --lib --target x86_64-apple-darwin

bindgen-swift:
	cargo build
	cargo run -p uniffi-bindgen generate --language swift --lib-file $(TARGET_DIR)/debug/libhello.dylib src/hello.udl
	sed -i '' 's/module\ HelloFFI/framework\ module\ HelloFFI/' src/HelloFFI.modulemap

bindgen-kotlin:
	cargo run -p uniffi-bindgen generate src/hello.udl --language kotlin -o platforms/android/UniffiRustExample/app/src/main/java
	sed -i '' 's/return "uniffi_Hello"/return "hello"/' platforms/android/UniffiRustExample/app/src/main/java/uniffi/Hello/Hello.kt

assemble-frameworks:
	cd $(TARGET_DIR) && find . -type d -name HelloFFI.framework -exec rm -rf {} \; || echo "rm failed"
	cd $(TARGET_DIR)/x86_64-apple-ios/debug && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && pwd && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/aarch64-apple-ios-sim/debug && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/aarch64-apple-ios/debug && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/aarch64-apple-ios-macabi/debug && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/x86_64-apple-ios-macabi/debug && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/x86_64-apple-darwin/debug && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources
	cd $(TARGET_DIR)/aarch64-apple-darwin/debug && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../hello/src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../hello/src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../hello/misc/apple/Info.plist ./Resources

xcframeowrk:
	lipo -create $(TARGET_DIR)/x86_64-apple-ios/debug/HelloFFI.framework/HelloFFI $(TARGET_DIR)/aarch64-apple-ios-sim/debug/HelloFFI.framework/HelloFFI -output $(TARGET_DIR)/aarch64-apple-ios-sim/debug/HelloFFI.framework/HelloFFI
	lipo -create $(TARGET_DIR)/x86_64-apple-ios-macabi/debug/HelloFFI.framework/HelloFFI $(TARGET_DIR)/aarch64-apple-ios-macabi/debug/HelloFFI.framework/HelloFFI -output $(TARGET_DIR)/aarch64-apple-ios-macabi/debug/HelloFFI.framework/HelloFFI
	lipo -create $(TARGET_DIR)/x86_64-apple-darwin/debug/HelloFFI.framework/HelloFFI $(TARGET_DIR)/aarch64-apple-darwin/debug/HelloFFI.framework/HelloFFI -output $(TARGET_DIR)/aarch64-apple-darwin/debug/HelloFFI.framework/HelloFFI
	rm -rf $(TARGET_DIR)/HelloFFI.xcframework || echo "skip removing"
	xcodebuild -create-xcframework -framework $(TARGET_DIR)/aarch64-apple-darwin/debug/HelloFFI.framework -framework $(TARGET_DIR)/aarch64-apple-ios/debug/HelloFFI.framework -framework $(TARGET_DIR)/aarch64-apple-ios-sim/debug/HelloFFI.framework -framework $(TARGET_DIR)/aarch64-apple-ios-macabi/debug/HelloFFI.framework -output $(TARGET_DIR)/HelloFFI.xcframework

cp-xcframeowrk-source:
	cp -r $(TARGET_DIR)/HelloFFI.xcframework platforms/apple/Hello/Sources
	cp src/Hello.swift platforms/apple/Hello/Sources/Hello

android:
	cd platforms/android/UniffiRustExample && ./gradlew cargoBuild --info
