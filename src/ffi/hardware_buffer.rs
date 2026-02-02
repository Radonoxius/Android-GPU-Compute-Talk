use std::ffi::c_void;

pub const GL_MAP_PERSISTENT_BIT_EXT: u32 = 0x0040;
pub const GL_MAP_COHERENT_BIT_EXT: u32 = 0x0080;
pub const GL_DYNAMIC_STORAGE_BIT_EXT: u32 = 0x0100;
pub const GL_CLIENT_STORAGE_BIT_EXT: u32 = 0x0200;

pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: u32 = 0x00004000;

unsafe extern "C" {
    ///Allocates a 1D Hardware buffer of specified byte-size
    ///on Android for GPU compute purposes
    /// 
    ///SAFETY: Returns `nullptr` on error or if
    ///**NOT SUPPORTED**
    pub fn alloc_hardware_buffer(
        hardware_buffer_size: u32
    ) -> *mut c_void;

    ///Maps the specified hardware buffer for reading/writing
    /// 
    ///SAFETY: Returns `nullptr` on error or if
    ///**NOT SUPPORTED**
    pub fn map_hardware_buffer(
        hardware_buffer: *mut c_void,
        content_size: i32
    ) -> *mut c_void;

    ///Unaps the specified hardware buffer
    /// 
    ///SAFETY: `hardware_buffer` must be valid!
    pub fn unmap_hardware_buffer(
        hardware_buffer: *mut c_void
    );

    ///De-allocates the specified hardware buffer
    /// 
    ///SAFETY: `hardware_buffer` must be valid!
    pub fn free_hardware_buffer(
        hardware_buffer: *mut c_void
    );

    ///Refer `EGL_ANDROID_get_native_client_buffer` EGL-extension
    pub fn eglGetNativeClientBufferANDROID(
        hardware_buffer: *mut c_void
    ) -> *mut c_void;

    ///Refer `GL_EXT_external_buffer` GLES-extension
    pub fn glBufferStorageExternalEXT(
        target: u32,
        offset: i64,
        size: isize,
        clientBuffer: *mut c_void,
        flags: u32
    );
}