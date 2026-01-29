unsafe extern "C" {
    ///Initializes an OpenGLES context
    ///on Android for GPU Compute
    ///#### USE IT ONLY ONCE, AT THE START OF THE PROGRAM!
    pub fn egl_init();

    ///Destroys the context created by
    ///`egl_init`
    ///#### USE IT ONLY ONCE, AT THE END OF THE PROGRAM!
    pub fn egl_terminate();
}