#version 310 es

precision highp float;

layout(local_size_x = 4, local_size_y = 1, local_size_z = 1) in;

layout(std430, binding = 0) readonly buffer A {
    float a[];
};

layout(std430, binding = 1) readonly buffer B {
    float b[];
};

layout(std430, binding = 2) writeonly buffer C {
    float c[];
};

void main() {
    uint index = gl_LocalInvocationID.x;

    c[index] = a[index] + b[index];
}