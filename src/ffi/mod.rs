use std::ffi::c_void;

pub mod egl_utils;
pub mod gles_utils;

pub mod mali_core_props;
pub mod hardware_buffer;

pub const GL_SHADER_STORAGE_BUFFER: u32 = 0x90D2;

pub const GL_STREAM_DRAW: u32 = 0x88E0;
pub const GL_STREAM_READ: u32 = 0x88E1;
pub const GL_STREAM_COPY: u32 = 0x88E2;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_STATIC_READ: u32 = 0x88E5;
pub const GL_STATIC_COPY: u32 = 0x88E6;
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const GL_DYNAMIC_READ: u32 = 0x88E9;
pub const GL_DYNAMIC_COPY: u32 = 0x88EA;

pub const GL_SHADER_STORAGE_BARRIER_BIT: u32 = 0x2000;

pub const GL_MAP_READ_BIT: u32 = 0x1;
pub const GL_MAP_WRITE_BIT: u32 = 0x2;

unsafe extern "C" {
    pub fn glGetIntegerv(pname: u32, data: *mut i32);

    pub fn glGetInteger64v(pname: u32, data: *mut i64);

    pub fn glGetIntegeri_v(pname: u32, index: u32, data: *mut i32);

    pub fn glGenBuffers(buffer_count: i32, buffers: *mut u32);

    pub fn glBindBuffer(buffer_type: u32, buffer: u32);

    pub fn glBindBufferBase(
        buffer_type: u32,
        buffer_index: u32, 
        buffer: u32
    );

    pub fn glBufferData(
        buffer_type: u32,
        size: isize,
        data: *const c_void,
        usage: u32
    );

    pub fn glDispatchCompute(x: u32, y: u32, z: u32);

    pub fn glMemoryBarrier(barriers: u32);

    pub fn glMapBufferRange(
        buffer_type: u32,
        offset: i64,
        size: isize,
        access: u32
    ) -> *mut c_void;

    pub fn glUnmapBuffer(buffer_type: u32);

    pub fn glDeleteBuffers(buffer_count: i32, buffers: *mut u32);


    pub(crate) fn getrandom(
        buffer: *mut c_void,
        size: usize,
        flags: u32
    ) -> isize;
}