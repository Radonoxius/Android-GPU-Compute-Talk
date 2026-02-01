# Android-GPU-Compute-Talk
WEC talk - GPU Compute on Android (using OpenGLES Compute)!

## Prerequisites
+ C programming (Rust is **not a strict requirement**)
+ An Android phone, running Android 11 or newer and
+ `Termux` app installed on that phone

## Performance Stats
On my phone (Mediatek Helio-G85 SOC and 4GB RAM), I ran
tests with 3 arrays (f32), each having 65535 x 384 elements (f32s) in all cases.
The array is around 96MiB each.

CPU variant:
>Compute time: ~140ms
>
>Total time:   ~1.2s
>
>RAM consumed: ~300MB

GPU variant:
>Compute time: ~39ms
>
>Total time:   ~1.3s
>
>RAM consumed: ~600MB

GPU variant-v2:
>Compute time: ~37ms
>
>Total time:   ~1.1s
>
>RAM consumed: ~300MB

When the Array has 262140 x 384 elements, we see the following numbers:

CPU variant:
>Compute time: ~600ms
>
>Total time:   ~4.8s
>
>RAM consumed: ~1.2GB

GPU variant: **Out of Memory**

GPU variant-v2:
>Compute time: ~135ms
>
>Total time:   ~4.2s
>
>RAM consumed: ~1.2GB

These values are device dependent. Its just for illustration!

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