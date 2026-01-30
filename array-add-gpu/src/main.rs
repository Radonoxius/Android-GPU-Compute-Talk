use std::{ffi::c_void, time::Instant};

use android_gpu_demos_lib::{
    GRND_URANDOM,
    
    ffi::{
        GL_SHADER_STORAGE_BARRIER_BIT,
        GL_SHADER_STORAGE_BUFFER,
        GL_STREAM_DRAW,
        egl_utils::{
            egl_init,
            egl_terminate
        },
        glBindBuffer,
        glBindBufferBase,
        glBufferData,
        glDeleteBuffers,
        glDispatchCompute, 
        glGenBuffers,
        glMemoryBarrier,
        gles_utils::{
            compile_shader,
            create_program,
            create_shader,
            gles_cleanup
        },
        mali_core_props::{
            CorePropertiesARM,
            get_core_properties_ARM, 
            glMaxActiveShaderCoresARM
        }
    },

    generate_random,
    read_shader
};

const ELEMENT_COUNT: usize = 384 * 65535;
const MAX_ACTIVE_CORES: u32 = 2;

fn main() {
    unsafe {
        egl_init();

        let _core_count =
            get_core_properties_ARM(CorePropertiesARM::CoreCount);

        glMaxActiveShaderCoresARM(
            MAX_ACTIVE_CORES
        );

        let t_start = Instant::now();

        let shader = create_shader(
            read_shader("shaders/array-add.comp.glsl")
        );

        compile_shader(shader);

        let program = create_program(shader);

        #[allow(non_snake_case)]
        let mut bufA = 0;
        glGenBuffers(1, &raw mut bufA);
        let a = generate_random::<f32>(ELEMENT_COUNT, GRND_URANDOM);

        #[allow(non_snake_case)]
        let mut bufB = 0;
        glGenBuffers(1, &raw mut bufB);
        let b = generate_random::<f32>(ELEMENT_COUNT, GRND_URANDOM);

        #[allow(non_snake_case)]
        let mut bufC = 0;
        glGenBuffers(1, &raw mut bufC);
        let c = Vec::<f32>::with_capacity(ELEMENT_COUNT * size_of::<f32>());

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufA);
        glBufferData(
            GL_SHADER_STORAGE_BUFFER,
            ELEMENT_COUNT as isize * 4,
            a.as_ptr() as *const c_void,
            GL_STREAM_DRAW
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, bufA);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufB);
        glBufferData(
            GL_SHADER_STORAGE_BUFFER,
            ELEMENT_COUNT as isize * 4,
            b.as_ptr() as *const c_void,
            GL_STREAM_DRAW
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 1, bufB);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufC);
        glBufferData(
            GL_SHADER_STORAGE_BUFFER,
            ELEMENT_COUNT as isize * 4,
            c.as_ptr() as *const c_void,
            GL_STREAM_DRAW
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, bufC);

        let t_compute_start = Instant::now();
        glDispatchCompute(65535, 1, 1);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufC);
        glMemoryBarrier(GL_SHADER_STORAGE_BARRIER_BIT);
        let t_compute_finish = t_compute_start.elapsed().as_millis();

        let t_finish = t_start.elapsed().as_millis();

        println!("Array has {} (f32) numbers!", ELEMENT_COUNT);
        println!("Total time: {}ms", t_finish);
        println!("Compute time: {}ms", t_compute_finish);

        glDeleteBuffers(1, &raw mut bufC);
        glDeleteBuffers(1, &raw mut bufB);
        glDeleteBuffers(1, &raw mut bufA);

        gles_cleanup(program, shader);
        egl_terminate();
    }
}