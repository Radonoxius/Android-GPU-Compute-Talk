#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include <string.h>

#include <GLES3/gl32.h>
#include <GLES3/gl3ext.h>

#include <EGL/egl.h>

EGLDisplay DISPLAY;
EGLContext CONTEXT;
EGLSurface SURFACE;

void egl_error_terminate(EGLDisplay egl_display) {
    eglTerminate(egl_display);
    eglReleaseThread();
    exit(1);
}

void egl_init(void) {
    EGLBoolean success;
    EGLint err;

    EGLDisplay egl_display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
    
    EGLint major;
    EGLint minor;
    success = eglInitialize(egl_display, &major, &minor);
    if(success != EGL_TRUE) {
        printf("An error occured while initializing display.\n");
        egl_error_terminate(egl_display);
    }

    char* vendor_name = (char*) eglQueryString(egl_display, EGL_VENDOR);
    printf("Initialized %s Display: EGL", vendor_name);
    printf("v%d.%d\n", major, minor);

    EGLint config_attrs[5] = {
        EGL_RENDERABLE_TYPE,
        EGL_OPENGL_ES3_BIT,
        EGL_CONFORMANT,
        EGL_OPENGL_ES3_BIT,
        EGL_NONE
    };

    EGLint total_config_count;
    eglChooseConfig(
        egl_display,
        config_attrs,
        NULL,
        0,
        &total_config_count
    );
    EGLConfig* all_configs = (EGLConfig*) malloc(
        sizeof(EGLConfig) * total_config_count
    );
    eglChooseConfig(
        egl_display,
        config_attrs,
        all_configs,
        total_config_count,
        &total_config_count
    );

    EGLConfig chosen_config = all_configs[0];
    free(all_configs);

    success = eglBindAPI(EGL_OPENGL_ES_API);
    if(success != EGL_TRUE) {
        printf("An error occured while binding to OpenGL ES API.\n");
        egl_error_terminate(egl_display);
    }

    EGLint egl_context_attrs[5] = {
        EGL_CONTEXT_MAJOR_VERSION,
        3,
        EGL_CONTEXT_MINOR_VERSION,
        2,
        EGL_NONE
    };

    EGLContext egl_context = eglCreateContext(
        egl_display,
        chosen_config,
        EGL_NO_CONTEXT,
        egl_context_attrs
    );
    err = eglGetError();
    if (egl_context == EGL_NO_CONTEXT) {
        printf("No EGL context available.\n");
        egl_error_terminate(egl_display);
    }
    if (err != EGL_SUCCESS) {
        printf("An error occured while creating the OpenGL ES context.\n");
        egl_error_terminate(egl_display);
    }

    uint8_t supports_surfaceless_ctx = 0;
    const char* egl_extensions = eglQueryString(egl_display, EGL_EXTENSIONS);
    char* result = strstr(egl_extensions, "EGL_KHR_surfaceless_context");

    if(result != NULL) {
        supports_surfaceless_ctx = 1;
        printf("Using EGL_KHR_surfaceless_context\n");
    }

    EGLSurface pbuffer = NULL;

    if(supports_surfaceless_ctx) {
        success = eglMakeCurrent(egl_display, EGL_NO_SURFACE, EGL_NO_SURFACE, egl_context);
        if(success != EGL_TRUE) {
            printf("An error occured while setting the current off-screen display.\n");
            eglMakeCurrent(egl_display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
            eglDestroyContext(egl_display, egl_context);
            egl_error_terminate(egl_display);
        }
    }
    else {
        pbuffer = eglCreatePbufferSurface(
            egl_display,
            chosen_config,
            NULL
        );
        err = eglGetError();
        if(err != EGL_SUCCESS) {
            printf("An error occured while creating the PBuffer\n");
            eglMakeCurrent(egl_display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
            eglDestroyContext(egl_display, egl_context);
            egl_error_terminate(egl_display);
        }
        printf("Using PBuffer\n");

        success = eglMakeCurrent(egl_display, pbuffer, pbuffer, egl_context);
        if(success != EGL_TRUE) {
            printf("An error occured while setting the current off-screen display.\n");
            eglMakeCurrent(egl_display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
            eglDestroySurface(egl_display, pbuffer);
            eglDestroyContext(egl_display, egl_context);
            egl_error_terminate(egl_display);
        }
    }

    DISPLAY = egl_display;
    CONTEXT = egl_context;
    SURFACE = pbuffer;
}

void egl_terminate(void) {
    eglMakeCurrent(DISPLAY, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
    if(SURFACE != NULL)
        eglDestroySurface(DISPLAY, SURFACE);
        
    eglTerminate(DISPLAY);
    eglReleaseThread();
}

//static void compute(const char* essl_comp_src, const int32_t comp_src_len) {
//    const unsigned char* vendor = (unsigned char*) glGetString(GL_VENDOR);
//    printf("%s\n", vendor);
//
//    GLuint shader = glCreateShader(GL_COMPUTE_SHADER);
//    GLenum err;
//
//    glShaderSource(shader, 1, &essl_comp_src, &comp_src_len);
//    err = glGetError();
//    if (err != GL_NO_ERROR) {
//        printf("An error occured while loading shader.\n");
//        glDeleteShader(shader);
//        exit(1);
//    }
//
//    glCompileShader(shader);
//    err = glGetError();
//    if (err != GL_NO_ERROR) {
//        printf("An error occured while compiling shader source.\n");
//        glDeleteShader(shader);
//        exit(1);
//    }
//
//    GLint params;
//    glGetShaderiv(shader, GL_COMPILE_STATUS, &params);
//    if (params != GL_TRUE) {
//        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &params);
//        char* log_msg = (char*) malloc(sizeof(char) * params);
//        glGetShaderInfoLog(shader, params, &params, log_msg);
//        printf("Shader compilation error:\n");
//        printf("%s\n", log_msg);
//    }
//
//    GLuint program = glCreateProgram();
//    glAttachShader(program, shader);
//    glLinkProgram(program);
//    glUseProgram(program);
//
//    GLuint ssbo[3];
//    glGenBuffers(3, ssbo);
//
//    float a[4] = {1.0f, 2.0f, 3.0f, 4.0f};
//    float b[4] = {4.0f, 3.0f, 2.0f, 1.0f};
//
//    float c[4];
//
//    // Buffer A (Binding 0)
//    glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo[0]);
//    glBufferData(GL_SHADER_STORAGE_BUFFER, 16, a, GL_STATIC_DRAW);
//    glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, ssbo[0]);
//
//    // Buffer B (Binding 1)
//    glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo[1]);
//    glBufferData(GL_SHADER_STORAGE_BUFFER, 16, b, GL_STATIC_DRAW);
//    glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 1, ssbo[1]);
//
//    // Buffer C (Binding 2 - Result)
//    glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo[2]);
//    glBufferData(GL_SHADER_STORAGE_BUFFER, 16, c, GL_STREAM_DRAW);
//    glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, ssbo[2]);
//
//    glDispatchCompute(1, 1, 1);
//
//    // Ensure all writes to the buffer are finished
//    glMemoryBarrier(GL_BUFFER_UPDATE_BARRIER_BIT);
//
//    // Map the buffer to read the data back to the CPU
//    glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo[2]);
//    float* ptr = (float*) glMapBufferRange(GL_SHADER_STORAGE_BUFFER, 0, 16, GL_MAP_READ_BIT);
//
//    // Use the data...
//    printf("[%f, %f, %f, %f]\n", ptr[0], ptr[1], ptr[2], ptr[3]);
//
//    glUnmapBuffer(GL_SHADER_STORAGE_BUFFER);
//    glDeleteBuffers(3, ssbo);
//    glDeleteProgram(program);
//    glDeleteShader(shader);
//}