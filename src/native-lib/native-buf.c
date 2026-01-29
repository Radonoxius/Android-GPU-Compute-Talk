#include "shared.h"

#include <android/hardware_buffer.h>

uint8_t IS_SUPPORTED_NATIVE_BUFFER = 0;

uint8_t _is_supported_ANDROID_hardware_buffer(void) {
    const char* egl_extensions = eglQueryString(DISPLAY, EGL_EXTENSIONS);
    char* res = strstr(egl_extensions, "EGL_ANDROID_get_native_client_buffer");
    char* dep1 = strstr(egl_extensions, "EGL_ANDROID_image_native_buffer");
    char* dep2 = strstr(egl_extensions, "EGL_KHR_image_base");

    const GLubyte* gl_extensions = glGetString(GL_EXTENSIONS);
    unsigned char* res2 = (unsigned char*)
        strstr((const char*) gl_extensions, "GL_EXT_external_buffer");
    unsigned char* dep3 = (unsigned char*)
        strstr((const char*) gl_extensions, "GL_EXT_buffer_storage");

    if (
        res != NULL &&
        dep1 != NULL &&
        dep2 != NULL &&

        res2 != NULL &&
        dep3 != NULL
    ) {
        IS_SUPPORTED_NATIVE_BUFFER = 1;
        return 1;
    }
    else
        return 0;
}

AHardwareBuffer* alloc_hardware_buffer(uint32_t buf_size) {
    if (
        IS_SUPPORTED_NATIVE_BUFFER ||
        _is_supported_ANDROID_hardware_buffer()
    ) {
        const AHardwareBuffer_Desc buf_desc = {
            .format = AHARDWAREBUFFER_FORMAT_BLOB,

            .usage = AHARDWAREBUFFER_USAGE_CPU_READ_MASK |
                AHARDWAREBUFFER_USAGE_CPU_READ_RARELY |
                AHARDWAREBUFFER_USAGE_CPU_WRITE_MASK |
                AHARDWAREBUFFER_USAGE_CPU_WRITE_RARELY |
                AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER,

            .rfu0 = 0,
            .rfu1 = 0,

            .height = 1,
            .width = buf_size,

            .layers = 1,
        };

        AHardwareBuffer* buf;
        int res = AHardwareBuffer_allocate(&buf_desc, &buf);

        if(res == 0)
            return buf;
        else
            return NULL;
    }
    else
        return NULL;
}

void* map_hardware_buffer(
    AHardwareBuffer *h_buffer,
    int32_t content_size
) {
    if (
        IS_SUPPORTED_NATIVE_BUFFER ||
        _is_supported_ANDROID_hardware_buffer()
    ) {
        const ARect mem_window = {
            .left = 0,
            .right = content_size,

            .top = 0,
            .bottom = 1
        };

        void* mapped_addrs;
        int res = AHardwareBuffer_lock(
            h_buffer,
            AHARDWAREBUFFER_USAGE_CPU_WRITE_RARELY,
            -1,
            &mem_window,
            &mapped_addrs
        );

        if (res == 0)
            return mapped_addrs;
        else
            return NULL;
    }
    else
        return NULL;
}

void unmap_hardware_buffer(
    AHardwareBuffer *h_buffer
) {
    if (
        IS_SUPPORTED_NATIVE_BUFFER ||
        _is_supported_ANDROID_hardware_buffer()
    )
        AHardwareBuffer_unlock(h_buffer, NULL);
}

void free_hardware_buffer(AHardwareBuffer* buffer) {
    if (
        IS_SUPPORTED_NATIVE_BUFFER ||
        _is_supported_ANDROID_hardware_buffer()
    )
        AHardwareBuffer_release(buffer);
}