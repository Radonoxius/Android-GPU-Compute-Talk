use std::ffi::c_void;

use android_gpu_demos_lib::{ffi::{GL_MAP_READ_BIT, GL_SHADER_STORAGE_BARRIER_BIT, GL_SHADER_STORAGE_BUFFER, GL_STREAM_DRAW, egl_utils::{egl_init, egl_terminate}, glBindBuffer, glBindBufferBase, glBufferData, glDeleteBuffers, glDispatchCompute, glGenBuffers, glMapBufferRange, glMemoryBarrier, glUnmapBuffer, gles_utils::{compile_shader, create_program, create_shader, gles_cleanup}, mali_core_props::{CorePropertiesARM, get_core_properties_ARM, glMaxActiveShaderCoresARM}}, read_shader};

fn main() {
    unsafe {
        egl_init();

        let _core_count =
            get_core_properties_ARM(CorePropertiesARM::CoreCount);

        glMaxActiveShaderCoresARM(
            1
        );

        let shader = create_shader(
            read_shader("shaders/test.comp.glsl")
        );

        compile_shader(shader);

        let program = create_program();

        #[allow(non_snake_case)]
        let mut bufA = 0;
        glGenBuffers(1, &raw mut bufA);
        let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];

        #[allow(non_snake_case)]
        let mut bufB = 0;
        glGenBuffers(1, &raw mut bufB);
        let b: [f32; 4] = [4.0, 3.0, 2.0, 1.0];

        #[allow(non_snake_case)]
        let mut bufC = 0;
        glGenBuffers(1, &raw mut bufC);
        let c: [f32; 4] = [0.0; 4];

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufA);
        glBufferData(
            GL_SHADER_STORAGE_BUFFER,
            16,
            a.as_ptr() as *const c_void,
            GL_STREAM_DRAW
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, bufA);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufB);
        glBufferData(
            GL_SHADER_STORAGE_BUFFER,
            16,
            b.as_ptr() as *const c_void,
            GL_STREAM_DRAW
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 1, bufB);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufC);
        glBufferData(
            GL_SHADER_STORAGE_BUFFER,
            16,
            c.as_ptr() as *const c_void,
            GL_STREAM_DRAW
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, bufC);

        glDispatchCompute(1, 1, 1);

        glMemoryBarrier(GL_SHADER_STORAGE_BARRIER_BIT);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufC);
        let addr = glMapBufferRange(
            GL_SHADER_STORAGE_BUFFER,
            0,
            16,
            GL_MAP_READ_BIT
        ) as *const f32;

        println!("{:?}\n", c);

        println!(
            "[{}, {}, {}, {}]",
            *addr,
            *(addr.wrapping_add(1)),
            *(addr.wrapping_add(2)),
            *(addr.wrapping_add(3))
        );

        glUnmapBuffer(GL_SHADER_STORAGE_BUFFER);

        glDeleteBuffers(1, &raw mut bufC);
        glDeleteBuffers(1, &raw mut bufB);
        glDeleteBuffers(1, &raw mut bufA);
        gles_cleanup(program, shader);
        egl_terminate();
    }
}