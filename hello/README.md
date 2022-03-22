# Hello example (minimal)

A shared library for the todo list application, which can be used for Android, iOS and other platforms.

## Prerequisites

Please refer to the root [README.md](../README.md).

## Binary Size

### iOS

- Generate binaries, xcframework and Swift binding: `make apple`

#### With Rust binary: 503K

```shell
# ls -lh
total 536K
drwxr-xr-x 4 _installd _installd  128 Jan  9 09:52 Base.lproj/
-rwxr-xr-x 1 _installd _installd 503K Jan  9 09:52 HelloAppleDemoApp*
-rw-r--r-- 1 _installd _installd 1.5K Jan  9 09:52 Info.plist
drwxr-xr-x 2 _installd _installd   64 Jan  9 09:52 META-INF/
-rw-r--r-- 1 _installd _installd    8 Jan  9 09:52 PkgInfo
drwxr-xr-x 3 _installd _installd   96 Jan  9 09:52 _CodeSignature/
-rw-r--r-- 1 _installd _installd  21K Jan  9 09:52 embedded.mobileprovision
```

#### Without Rust binary: 147K

```shell
# ls -lh
total 180K
drwxr-xr-x 4 _installd _installd  128 Jan  9 09:57 Base.lproj/
-rwxr-xr-x 1 _installd _installd 147K Jan  9 09:57 HelloAppleDemoApp*
-rw-r--r-- 1 _installd _installd 1.5K Jan  9 09:57 Info.plist
drwxr-xr-x 2 _installd _installd   64 Jan  9 09:57 META-INF/
-rw-r--r-- 1 _installd _installd    8 Jan  9 09:57 PkgInfo
drwxr-xr-x 3 _installd _installd   96 Jan  9 09:57 _CodeSignature/
-rw-r--r-- 1 _installd _installd  21K Jan  9 09:52 embedded.mobileprovision
```
