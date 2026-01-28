use android_gpu_demos_lib::ffi::egl_utils::{egl_init, egl_terminate};

fn main() {
    unsafe {
        egl_init();

        println!("Hello from rust!");

        egl_terminate();
    }
}