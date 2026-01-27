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

CFLAGS = -O3 -Wall
LIBS = -lGLESv3 -lEGL

BUILD_DIR = build
RUST_BUILD_DIR = target

SRC_DIR = src/native-lib
SHADER_SRC_DIR = shaders

SOURCES = $(wildcard $(SRC_DIR)/*.c)
TARGET = $(BUILD_DIR)/main.elf

all: $(TARGET) rust_build copy_shaders

$(TARGET): $(SOURCES)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CTARGET) $(SYSROOT) $(SOURCES) $(CFLAGS) $(LIBS) $(XFLAGS) -o $(TARGET)

rust_build: $(TARGET)
	@NDK_SYSROOT="$(NDK_SYSROOT)" \
	CC_RES_DIR="$(CC_RES_DIR)" \
	cargo br

copy_shaders:
	@mkdir -p $(BUILD_DIR)/$(SHADER_SRC_DIR)
	@cp -r $(SHADER_SRC_DIR)/* $(BUILD_DIR)/$(SHADER_SRC_DIR)/

clean:
	@rm -rf $(BUILD_DIR)
	@rm -rf $(RUST_BUILD_DIR)

.PHONY: all rust_build copy_shaders clean