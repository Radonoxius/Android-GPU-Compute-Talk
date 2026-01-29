use android_gpu_demos_lib::ffi::{egl_utils::{egl_init, egl_terminate}, mali_core_props::{CorePropertiesARM, get_core_properties_ARM, glMaxActiveShaderCoresARM}};

fn main() {
    egl_init();

    unsafe {
        let core_count =
            get_core_properties_ARM(CorePropertiesARM::CoreCount);

        glMaxActiveShaderCoresARM(
            core_count.unwrap_or(1) as u32
        );
    }

    println!("Hello from rust!");

    egl_terminate();
}