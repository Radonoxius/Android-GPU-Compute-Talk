precision highp float;

layout(local_size_x = WORKGROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

layout(std430, binding = 0) readonly restrict buffer A {
    vec4 a[];
};

layout(std430, binding = 1) readonly restrict buffer B {
    vec4 b[];
};

layout(std430, binding = 2) writeonly restrict buffer C {
    vec4 c[];
};

void main() {
    uint index = gl_GlobalInvocationID.x;

    c[index] = a[index] + b[index];
}