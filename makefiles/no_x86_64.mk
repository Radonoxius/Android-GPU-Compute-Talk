include Makefile

NDK_SYSROOT = $(NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64/sysroot/

CC = clang

CTARGET = --target=aarch64-linux-android30

SYSROOT = --sysroot=$(NDK_SYSROOT)
XFLAGS = -lc -lm -nodefaultlibs -rtlib=compiler-rt

build: raw_compile copy_shaders

raw_compile: $(SOURCES)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CTARGET) $(SYSROOT) $(SOURCES) $(CFLAGS) $(LIBS) $(XFLAGS) -o $(TARGET)

.PHONY: build raw_compile