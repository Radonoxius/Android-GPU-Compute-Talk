# Set the NDK_HOME path to the actual path before running!
NDK_HOME = /home/radon/Android/Sdk/ndk/27.3.13750724

CC = $(NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang

CFLAGS = -O3 -fPIC -Wall
LIBS = -lGLESv3 -lEGL

BUILD_DIR = build
SRC_DIR = native

SOURCES = $(wildcard $(SRC_DIR)/*.c)
TARGET = $(BUILD_DIR)/main

all: $(TARGET) copy_shaders

$(TARGET): $(SOURCES)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(SOURCES) $(CFLAGS) $(LIBS) -o $(TARGET)

copy_shaders:
	@mkdir -p $(BUILD_DIR)/shaders
	cp -r shaders/* $(BUILD_DIR)/shaders/

clean:
	rm -rf $(BUILD_DIR)

# Use `make no_x64` to build on non-x86_64 linux platforms
# Requires clang!
no_x86_64:
	$(MAKE) -f makefiles/no_x86_64.mk build

.PHONY: all copy_shaders clean no_x86_64
