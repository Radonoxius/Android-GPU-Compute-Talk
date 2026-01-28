pub mod ffi {
    pub mod egl_utils {
        unsafe extern "C" {
            ///Initializes an OpenGLES context
            ///on Android for GPU Compute
            ///#### USE IT ONLY ONCE!
            pub fn egl_init();

            ///Destroys the context created by
            ///`egl_init`
            ///#### USE IT ONLY ONCE!
            pub fn egl_terminate();
        }
    }
}