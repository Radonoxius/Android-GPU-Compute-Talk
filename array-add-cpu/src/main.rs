use std::{
    io::stdin,
    time::Instant
};

use android_gpu_utils::{
    GRND_URANDOM,

    ffi::{
        egl_utils::{
            egl_init,
            egl_terminate
        },
        gles_utils::get_max_work_group_invocations
    },
    
    generate_random
};

fn main() {
    unsafe {
        egl_init();
    }

    let max_lwg_invocations = get_max_work_group_invocations();
    println!("Local Work Group size is: {}\n", max_lwg_invocations);

    let mut input = String::new();
    println!("Enter the CPU-Global Work Group size (unrestricted)");
    stdin().read_line(&mut input).unwrap();
    input.pop();
    let gwg_size: usize = input.parse().unwrap();

    let t_start = Instant::now();

    let a: Vec<f32>;
    let b: Vec<f32>;
    let mut c: Vec<f32>;

    let n = gwg_size * max_lwg_invocations as usize;

    println!("\nOperating on {} (f32) numbers!\n", n);

    a = generate_random(
        n,
        GRND_URANDOM
    );

    b = generate_random(
        n,
        GRND_URANDOM
    );

    c = Vec::with_capacity(n * 4);

    let t_compute_start = Instant::now();
    for i in 0..(a.len()) {
        c.push(a[i] + b[i]);
    }
    let t_compute_finish = t_compute_start.elapsed().as_micros();
    let t_finish = t_start.elapsed().as_millis();

    println!("Compute time: {}micros", t_compute_finish);
    println!("Total time: {}ms", t_finish);

    unsafe {
        egl_terminate();
    }
}