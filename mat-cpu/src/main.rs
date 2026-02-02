use std::io::stdin;
use std::ffi::c_void;
use std::time::Instant;

use android_gpu_utils::{
    GRND_URANDOM,

    ffi::{
        egl_utils::{
            egl_init,
            egl_terminate
        },
        gles_utils::get_max_work_group_invocations
    },

    fill_random
};

/// Aligned to 16 bytes to ensure the compiler can use 128-bit SIMD loads safely.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
struct Mat4 {
    // We use a nested array to help the compiler see the row-major structure
    rows: [[f32; 4]; 4],
}

impl Mat4 {
    /// The "Golden Loop" - Cross-platform optimized matrix multiplication.
    /// By using fixed-size unrolled loops, LLVM can auto-vectorize this 
    /// into SSE, AVX, or NEON instructions.
    #[inline(always)]
    fn multiply(a: &Mat4, b: &Mat4) -> Mat4 {
        let mut out = [[0.0; 4]; 4];
        
        // This pattern (Linear Combination) is "Fast as Hell" because it
        // broadcasts a single value from A across a whole row of B.
        for i in 0..4 {
            for j in 0..4 {
                let scalar = a.rows[i][j];
                // This inner loop is 100% auto-vectorized by the compiler.
                for k in 0..4 {
                    out[i][k] += scalar * b.rows[j][k];
                }
            }
        }
        Mat4 { rows: out }
    }

    ///Gives a zero matrix
    #[inline(always)]
    fn zero() -> Self {
        Mat4 {
            rows: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

    ///Generates a matrix that has random values
    #[inline(always)]
    fn random() -> Self {
        let mut m = Mat4::zero();

        unsafe {
            fill_random::<f32>(
                &raw mut m.rows[0][0] as *mut c_void,
                16,
                GRND_URANDOM
            );
        }

        m
    }
}

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
    let n: usize = input.parse::<usize>().unwrap() * max_lwg_invocations as usize;

    let mut matrices1 = Vec::with_capacity(n);
    let mut matrices2 = Vec::with_capacity(n);
    let mut res = Vec::with_capacity(n);

    println!("{:?}", Mat4::random());

    for _ in 0..n {
        matrices1.push(Mat4::random());
        matrices2.push(Mat4::random());
    }

    println!("\nOperating on {} 4x4-f32 matrices!\n", n);

    let start = Instant::now();
    
    for i in 0..n {
        res.push(Mat4::multiply(&matrices1[i], &matrices2[i]));
    }
    
    let duration = start.elapsed().as_micros();
    println!("Compute time: {}micros", duration);

    unsafe {
        egl_terminate();
    }
}