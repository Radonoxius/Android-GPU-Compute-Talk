#include "lib.c"

int main(void) {
    EGLDisplay display = gles_init();
    
    FILE* handle = fopen("shaders/test.comp.glsl", "r");

    fseek(handle, 0, SEEK_END);
    long size = ftell(handle) + 1;
    char* essl_comp_src = (char*) malloc(sizeof(char) * size);
    fseek(handle, 0, SEEK_SET);

    fread(essl_comp_src, sizeof(char), size, handle);
    fclose(handle);

    compute(essl_comp_src, size);

    free(essl_comp_src);
    
    eglMakeCurrent(display, EGL_NO_DISPLAY, EGL_NO_DISPLAY, EGL_NO_CONTEXT);
    eglReleaseThread();

    eglTerminate(display);
    return 0;
}