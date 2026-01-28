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
+ GNU `make`
+ Android NDK (LTS version)
+ `clang`, `llvm-ar`, `lld`, `rustc` (2024 edition) and `cargo`

Before building, **you must** update the NDK_HOME variable in the Makefile. 
After that, run the following to build everything:
```
make
```

All the executables can be found in `target/aarch64-linux-android/release` folder.
You will also need the `shaders` folder for the programs to work properly.

NOTE: The executables run ONLY on Android-Aarch64 devices!