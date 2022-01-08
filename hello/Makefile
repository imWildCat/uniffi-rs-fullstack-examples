STATIC_LIB_NAME := libhello.a

build-targets:
	cargo build --release --target x86_64-apple-ios
	cargo build --release --target aarch64-apple-ios-sim
	cargo build --release --target aarch64-apple-ios

bindgen-swift:
	uniffi-bindgen generate src/hello.udl --language swift

assemble-frameworks:
	find . -type d -name HelloFFI.framework -exec rm -rf {} \;
	cd target/x86_64-apple-ios/release && mkdir HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources
	cd target/aarch64-apple-ios-sim/release && mkdir HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources
	cd target/aarch64-apple-ios/release && mkdir HelloFFI.framework && cd HelloFFI.framework && mkdir Headers Modules Resources && cp ../../../../src/HelloFFI.modulemap ./Modules/module.modulemap && cp ../../../../src/HelloFFI.h ./Headers/HelloFFI.h && cp ../$(STATIC_LIB_NAME) ./HelloFFI && cp ../../../../misc/apple/Info.plist ./Resources

xcframeowrk:
	lipo -create target/x86_64-apple-ios/release/HelloFFI.framework/HelloFFI target/aarch64-apple-ios-sim/release/HelloFFI.framework/HelloFFI -output target/aarch64-apple-ios-sim/release/HelloFFI.framework/HelloFFI
	xcodebuild -create-xcframework -framework target/aarch64-apple-ios/release/HelloFFI.framework -framework target/aarch64-apple-ios-sim/release/HelloFFI.framework -output target/HelloFFI.xcframework

cp-xcframeowrk-source:
	cp -r target/HelloFFI.xcframework platforms/HelloAppleDemoApp
	cp src/Hello.swift platforms/HelloAppleDemoApp
