use android_gpu_demos_lib::ffi::{egl_utils::{egl_init, egl_terminate}, mali_core_props::{CorePropertiesARM, get_core_properties_ARM, glMaxActiveShaderCoresARM}};

fn main() {
    egl_init();

    unsafe {
        let val =
            get_core_properties_ARM(CorePropertiesARM::ActiveCoreCount);
        dbg!(val);

        glMaxActiveShaderCoresARM(1);
    }

    println!("Hello from rust!");

    egl_terminate();
}