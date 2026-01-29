use std::ptr::null_mut;

use android_gpu_demos_lib::ffi::{egl_utils::{egl_init, egl_terminate}, hardware_buffer::{alloc_hardware_buffer, free_hardware_buffer, map_hardware_buffer, unmap_hardware_buffer}, mali_core_props::{CorePropertiesARM, get_core_properties_ARM, glMaxActiveShaderCoresARM}};

fn main() {
    egl_init();

    unsafe {
        let core_count =
            get_core_properties_ARM(CorePropertiesARM::CoreCount);

        glMaxActiveShaderCoresARM(
            core_count.unwrap_or(1) as u32
        );

        let hb = alloc_hardware_buffer(4);
        if hb != null_mut() {
            let m_addr =
            map_hardware_buffer(hb, 4) as *mut f32;
            *m_addr = 3.14;
            unmap_hardware_buffer(hb);

            let v =
                map_hardware_buffer(hb, 4) as *mut f32;
            println!("{}", *v);
            unmap_hardware_buffer(hb);

            free_hardware_buffer(hb);
        }
    }

    println!("Hello from rust!");

    egl_terminate();
}