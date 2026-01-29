#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include <string.h>

#include <GLES3/gl32.h>
#include <GLES3/gl3ext.h>

#include <EGL/egl.h>

uint8_t _is_supported_ARM_core_properties() {
    const GLubyte* gl_extensions = glGetString(GL_EXTENSIONS);
    unsigned char* result = (unsigned char*)
        strstr((const char*) gl_extensions, "GL_ARM_shader_core_properties");

    if (result != NULL)
        return 1;
    else
        return 0;
}

void* _setup_glMaxActiveShaderCoresARM() {
    return eglGetProcAddress("glMaxActiveShaderCoresARM");
}