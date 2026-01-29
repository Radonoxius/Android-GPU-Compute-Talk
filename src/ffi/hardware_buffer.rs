use std::ffi::c_void;

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