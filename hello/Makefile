STATIC_LIB_NAME := libhello.a

apple:
	@make build-targets
	@make bindgen-swift
	@make assemble-frameworks
	@make xcframeowrk
	@make cp-xcframeowrk-source

build-targets:
	cargo build --release --target x86_64-apple-ios
	cargo +nightly build --release --target aarch64-apple-ios-sim
	cargo +nightly build -Z build-std --release --target aarch64-apple-ios-macabi
	cargo +nightly build -Z build-std --release --target x86_64-apple-ios-macabi
	cargo build --release --target aarch64-apple-ios

bindgen-swift:
	uniffi-bindgen generate src/hello.udl --language swift
	sed -i '' 's/module\ HelloFFI/framework\ module\ HelloFFI/' src/HelloFFI.modulemap

bindgen-kotlin:
	uniffi-bindgen generate src/hello.udl --language kotlin -o platforms/android/UniffiRustExample/app/src/main/java
	sed -i '' 's/return "uniffi_Hello"/return "hello"/' platforms/android/UniffiRustExample/app/src/main/java/uniffi/Hello/Hello.kt

assemble-frameworks:
	find . -type d -name HelloFFI.framework -exec rm -rf {} \; || echo "rm failed"
	cd target/x86_64-apple-ios/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources
	cd target/aarch64-apple-ios-sim/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources
	cd target/aarch64-apple-ios/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources
	cd target/aarch64-apple-ios-macabi/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources
	cd target/x86_64-apple-ios-macabi/release && mkdir -p HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources

xcframeowrk:
	lipo -create target/x86_64-apple-ios/release/HelloFFI.framework/HelloFFI target/aarch64-apple-ios-sim/release/HelloFFI.framework/HelloFFI -output target/aarch64-apple-ios-sim/release/HelloFFI.framework/HelloFFI
	lipo -create target/x86_64-apple-ios-macabi/release/HelloFFI.framework/HelloFFI target/aarch64-apple-ios-macabi/release/HelloFFI.framework/HelloFFI -output target/aarch64-apple-ios-macabi/release/HelloFFI.framework/HelloFFI
	rm -rf target/HelloFFI.xcframework || echo "skip removing"
	xcodebuild -create-xcframework -framework target/aarch64-apple-ios/release/HelloFFI.framework -framework target/aarch64-apple-ios-sim/release/HelloFFI.framework -framework target/aarch64-apple-ios-macabi/release/HelloFFI.framework -output target/HelloFFI.xcframework

cp-xcframeowrk-source:
	cp -r target/HelloFFI.xcframework platforms/apple/Hello/Sources
	cp src/Hello.swift platforms/apple/Hello/Sources/Hello

android:
	cd hello/platforms/android/UniffiRustExample && ./gradlew cargoBuild --info
