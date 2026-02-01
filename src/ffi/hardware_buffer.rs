use std::ffi::c_void;

pub const GL_MAP_PERSISTENT_BIT_EXT: u32 = 0x0040;
pub const GL_MAP_COHERENT_BIT_EXT: u32 = 0x0080;
pub const GL_DYNAMIC_STORAGE_BIT_EXT: u32 = 0x0100;
pub const GL_CLIENT_STORAGE_BIT_EXT: u32 = 0x0200;

pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: u32 = 0x00004000;

unsafe extern "C" {
    pub fn alloc_hardware_buffer(
        hardware_buffer_size: u32
    ) -> *mut c_void;

    pub fn map_hardware_buffer(
        hardware_buffer: *mut c_void,
        content_size: i32
    ) -> *mut c_void;

    pub fn unmap_hardware_buffer(
        hardware_buffer: *mut c_void
    );

    pub fn free_hardware_buffer(
        hardware_buffer: *mut c_void
    );

    pub fn eglGetNativeClientBufferANDROID(
        hardware_buffer: *mut c_void
    ) -> *mut c_void;

    pub fn glBufferStorageExternalEXT(
        target: u32,
        offset: i64,
        size: isize,
        clientBuffer: *mut c_void,
        flags: u32
    );
}