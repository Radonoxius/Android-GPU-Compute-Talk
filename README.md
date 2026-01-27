# Android-GPU-Compute-Talk
WEC talk - GPU Compute on Android (using OpenGLES Compute)!

## Prerequisites
+ C programming (Rust is **not a strict requirement**)
+ An Android phone, running Android 11 or newer and
+ `Termux` app installed on that phone

## Build prerequisites
I recommend you to download the pre-compiled binaries which can be found in the releases section.

However, if you want to build/compile this project on your machine, youll need:

+ Linux or MacOS
+ GNU Make
+ Android NDK (LTS version)
+ `clang`, `llvm-ar`, `lld`, `rustc` (2024 edition) and `cargo`

Just run the following to build everything:
```
make
```