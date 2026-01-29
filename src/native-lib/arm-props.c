#include "shared.h"

uint8_t _is_supported_ARM_core_properties(void) {
    const GLubyte* gl_extensions = glGetString(GL_EXTENSIONS);
    unsigned char* result = (unsigned char*)
        strstr((const char*) gl_extensions, "GL_ARM_shader_core_properties");

    if (result != NULL)
        return 1;
    else
        return 0;
}

void* _setup_glMaxActiveShaderCoresARM(void) {
    return eglGetProcAddress("glMaxActiveShaderCoresARM");
}