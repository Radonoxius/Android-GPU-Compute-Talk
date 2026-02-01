use std::{
    env,
    ffi::c_void,
    io::stdin,
    str::FromStr,
    time::Instant
};

use android_gpu_demos_lib::{
    GRND_URANDOM,
    
    ffi::{
        GL_SHADER_STORAGE_BARRIER_BIT, GL_SHADER_STORAGE_BUFFER, GL_STATIC_READ, GL_STREAM_DRAW, egl_utils::{
            egl_init,
            egl_terminate
        }, glBindBuffer, glBindBufferBase, glBufferData, glDeleteBuffers, glDispatchCompute, glFinish, glGenBuffers, glMemoryBarrier, gles_utils::{
            compile_shader,
            create_program,
            create_shader,
            get_max_work_group_count, 
            get_max_work_group_invocations,
            gles_cleanup
        }, mali_core_props::glMaxActiveShaderCoresARM
    },

    generate_random,
    read_shader
};

static mut ELEMENT_COUNT: usize = 0;

#[allow(static_mut_refs)]
fn main() {
    unsafe {
        egl_init();

        let max_core_count = env::args().nth(1)
            .unwrap_or(String::from_str("2").unwrap())
            .parse().unwrap();

        glMaxActiveShaderCoresARM(
            max_core_count
        );

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

        ELEMENT_COUNT = gwg_size * max_lwg_invocations;

        let t_start = Instant::now();

        let shader = create_shader(
            read_shader("shaders/array-add.comp.glsl")
        );

        compile_shader(shader);

        let program = create_program(shader);

        println!("\nOperating on {} (f32) numbers!\n", ELEMENT_COUNT);

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
        let c = Vec::<f32>::with_capacity(ELEMENT_COUNT * 4);

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
            GL_STATIC_READ
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

        gles_cleanup(program, shader);

        let t_finish = t_start.elapsed().as_millis();
        println!("Total time: {}ms", t_finish);

        egl_terminate();
    }
}