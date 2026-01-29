#include "shared.h"

GLuint create_shader_from_src(
    const GLchar *const src_code,
    const GLint len
) {
    GLuint shader = glCreateShader(GL_COMPUTE_SHADER);

    GLenum err;
    glShaderSource(shader, 1, &src_code, &len);
    err = glGetError();
    if (err != GL_NO_ERROR) {
        printf("An error occured while loading shader.\n");
        glDeleteShader(shader);
        exit(1);
    }

    return shader;
}

void compile_shader(GLuint shader) {
    GLenum err;

    glCompileShader(shader);
    err = glGetError();
    if (err != GL_NO_ERROR) {
        printf("An error occured while compiling shader source.\n");
        glDeleteShader(shader);
        exit(1);
    }

    GLint params;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &params);
    if (params != GL_TRUE) {
        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &params);
        char* log_msg = (char*) malloc(sizeof(char) * params);
        glGetShaderInfoLog(shader, params, &params, log_msg);
        printf("Shader compilation error:\n");
        printf("%s\n\n", log_msg);
        free(log_msg);
    }
}

GLuint create_program(GLuint shader) {
    GLuint program = glCreateProgram();

    glAttachShader(program, shader);
    glLinkProgram(program);
    glUseProgram(program);

    return program;
}

void gles_cleanup(GLuint program, GLuint shader) {
    glDeleteProgram(program);
    glDeleteShader(shader);
}