use std::{
    io::stdin,
    time::Instant
};

use android_gpu_utils::{
    GRND_URANDOM,
    
    ffi::{
        GL_MAP_READ_BIT,
        GL_MAP_WRITE_BIT,
        GL_SHADER_STORAGE_BARRIER_BIT,
        GL_SHADER_STORAGE_BUFFER,
        egl_utils::{
            egl_init,
            egl_terminate
        },
        glBindBuffer,
        glBindBufferBase,
        glDeleteBuffers, 
        glDispatchCompute,
        glFinish,
        glGenBuffers,
        glMemoryBarrier,
        gles_utils::{
            compile_shader,
            create_program,
            create_shader,
            get_max_work_group_count, 
            get_max_work_group_invocations,
            gles_cleanup
        },
        hardware_buffer::{
            GL_CLIENT_STORAGE_BIT_EXT,
            alloc_hardware_buffer,
            eglGetNativeClientBufferANDROID,
            free_hardware_buffer,
            glBufferStorageExternalEXT,
            map_hardware_buffer,
            unmap_hardware_buffer
        },
    },
    
    fill_random,
    read_shader
};

static mut ELEMENT_COUNT: usize = 0;

#[allow(static_mut_refs)]
fn main() {
    unsafe {
        egl_init();

        let max_lwg_invocations = get_max_work_group_invocations() as usize;
        println!("GPU Maximum Local Work Group Invocations is: {}", max_lwg_invocations);
        println!(
            "GPU Maximum Global Work Group size is: {}\n",
            get_max_work_group_count().0
        );

        let mut input = String::new();
        println!("Enter the Global Work Group size");
        stdin().read_line(&mut input).unwrap();
        input.pop();
        let gwg_size: usize = input.parse().unwrap();

        ELEMENT_COUNT = gwg_size * max_lwg_invocations * 4;

        let t_start = Instant::now();

        let shader = create_shader(
            read_shader("shaders/array-add-v2.comp.glsl")
        );

        compile_shader(shader);

        let program = create_program(shader);

        println!("\nOperating on {} (f32) numbers!\n", ELEMENT_COUNT);

        #[allow(non_snake_case)]
        let mut bufA = 0;
        glGenBuffers(1, &raw mut bufA);
        let a =
            alloc_hardware_buffer(ELEMENT_COUNT as u32 * 4);
        let a_mapped =
            map_hardware_buffer(a, ELEMENT_COUNT as i32 * 4);
        fill_random::<f32>(a_mapped, ELEMENT_COUNT, GRND_URANDOM);
        unmap_hardware_buffer(a);
        let a_egl_buffer = eglGetNativeClientBufferANDROID(a);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufA);
        glBufferStorageExternalEXT(
            GL_SHADER_STORAGE_BUFFER,
            0,
            ELEMENT_COUNT as isize * 4,
            a_egl_buffer,
            GL_MAP_WRITE_BIT | GL_CLIENT_STORAGE_BIT_EXT
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, bufA);

        #[allow(non_snake_case)]
        let mut bufB = 0;
        glGenBuffers(1, &raw mut bufB);
        let b =
            alloc_hardware_buffer(ELEMENT_COUNT as u32 * 4);
        let b_mapped =
            map_hardware_buffer(b, ELEMENT_COUNT as i32 * 4);
        fill_random::<f32>(b_mapped, ELEMENT_COUNT, GRND_URANDOM);
        unmap_hardware_buffer(b);
        let b_egl_buffer = eglGetNativeClientBufferANDROID(b);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufB);
        glBufferStorageExternalEXT(
            GL_SHADER_STORAGE_BUFFER,
            0,
            ELEMENT_COUNT as isize * 4,
            b_egl_buffer,
            GL_MAP_WRITE_BIT | GL_CLIENT_STORAGE_BIT_EXT
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 1, bufB);

        #[allow(non_snake_case)]
        let mut bufC = 0;
        glGenBuffers(1, &raw mut bufC);
        let c =
            alloc_hardware_buffer(ELEMENT_COUNT as u32 * 4);
        let c_egl_buffer = eglGetNativeClientBufferANDROID(c);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufC);
        glBufferStorageExternalEXT(
            GL_SHADER_STORAGE_BUFFER,
            0,
            ELEMENT_COUNT as isize * 4,
            c_egl_buffer,
            GL_MAP_READ_BIT | GL_CLIENT_STORAGE_BIT_EXT
        );
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, bufC);

        let t_compute_start = Instant::now();
        glDispatchCompute(gwg_size as u32, 1, 1);

        glBindBuffer(GL_SHADER_STORAGE_BUFFER, bufC);
        glMemoryBarrier(GL_SHADER_STORAGE_BARRIER_BIT);
        glFinish();
        
        let t_compute_finish = t_compute_start.elapsed().as_micros();

        println!("Compute time: {}micros", t_compute_finish);

        glDeleteBuffers(1, &raw mut bufC);
        glDeleteBuffers(1, &raw mut bufB);
        glDeleteBuffers(1, &raw mut bufA);

        free_hardware_buffer(c);
        free_hardware_buffer(b);
        free_hardware_buffer(a);

        gles_cleanup(program, shader);

        let t_finish = t_start.elapsed().as_millis();
        println!("Total time: {}ms", t_finish);

        egl_terminate();
    }
}