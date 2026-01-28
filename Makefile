# Windows is not supported!

# Set the NDK_HOME path to the actual path before running!
NDK_HOME = /home/radon/Android/Sdk/ndk/27.3.13750724

PLATFORM := $(shell uname -s)

ifeq ($(PLATFORM), Linux)
    NDK_SYSROOT := $(NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64/sysroot/
	CC_RES_DIR := $(NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64/lib/clang/18
else
    NDK_SYSROOT := $(NDK_HOME)/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/
	CC_RES_DIR := $(NDK_HOME)/toolchains/llvm/prebuilt/darwin-x86_64/lib/clang/18
endif

CC = clang

CTARGET = --target=aarch64-linux-android30

SYSROOT = --sysroot=$(NDK_SYSROOT)
XFLAGS = -resource-dir=$(CC_RES_DIR) \
	-fPIC -fdata-sections -ffunction-sections -funwind-tables -fstack-protector-strong \
	-no-canonical-prefixes -fvisibility=hidden -fvisibility-inlines-hidden \
	-ffunction-sections -fdata-sections -march=armv8.2-a -rtlib=compiler-rt

CFLAGS = -c -O3 -Wall -Werror

RUST_BUILD_DIR = target
BUILD_DIR = $(RUST_BUILD_DIR)/aarch64-linux-android/release/deps

SRC_DIR = src/native-lib
SHADER_SRC_DIR = shaders

SOURCES = $(wildcard $(SRC_DIR)/*.c)
TARGET = $(BUILD_DIR)/egl-init.o

LIB_TARGET = $(BUILD_DIR)/libutils.a

all: $(LIB_TARGET) rust_build

$(LIB_TARGET): $(SOURCES)
	@mkdir -p $(BUILD_DIR)
	@$(CC) $(CTARGET) $(SYSROOT) $(SOURCES) $(CFLAGS) $(XFLAGS) -o $(TARGET)
	@llvm-ar -rcs $(LIB_TARGET) $(TARGET)

rust_build: $(TARGET)
	@NDK_SYSROOT="$(NDK_SYSROOT)" \
	CC_RES_DIR="$(CC_RES_DIR)" \
	cargo br

clean:
	@rm -rf $(BUILD_DIR)
	@rm -rf $(RUST_BUILD_DIR)

.PHONY: all rust_build clean