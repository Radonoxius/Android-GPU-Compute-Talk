precision highp float;

layout(local_size_x = WORKGROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

layout(std430, binding = 0, row_major) readonly restrict buffer A {
    mat4 a[];
};

layout(std430, binding = 1, row_major) readonly restrict buffer B {
    mat4 b[];
};

layout(std430, binding = 2, row_major) writeonly restrict buffer C {
    mat4 c[];
};

void main() {
    uint index = gl_GlobalInvocationID.x;

    c[index] = a[index] * b[index];
}