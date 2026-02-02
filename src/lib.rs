pub mod ffi;

use std::{
    fs::OpenOptions,
    io::Read,
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign, 
        Mul,
        MulAssign,
        Sub,
        SubAssign
    },
    os::raw::c_void
};

use crate::ffi::{
    getrandom,
    gles_utils::get_max_work_group_invocations
};

///Reads the shader at the specified path and
///adds the following lines to the top of the shader src:
/// 
///#version 320 es<br>  
///#define WORKGROUP_SIZE x
/// 
///where, x is the maximum local work group
///invocations (determined at runtime)
/// 
///Finally, returns the shader source code as
///a vector of bytes
pub fn read_shader(path: &str) -> Vec<u8> {
    let max_lwg_invocations = get_max_work_group_invocations();

    let mut header = format!(
        "#version 320 es\n#define WORKGROUP_SIZE {}\n\n",
        max_lwg_invocations
    );

    let mut shader_bytes: Vec<u8>;
    unsafe {
        shader_bytes = Vec::from(header.as_bytes_mut());
    }

    let mut src_bytes = Vec::new();

    let mut shader = OpenOptions::new()
        .read(true).open(path).unwrap();

    shader.read_to_end(&mut src_bytes).unwrap();

    shader_bytes.append(&mut src_bytes);

    shader_bytes
}

pub const GRND_INSECURE: u32 = 0x4;
pub const GRND_RANDOM: u32 = 0x2;
pub const GRND_URANDOM: u32 = 0x0;
pub const GRND_NONBLOCK: u32 = 0x1;

///Returns a vector of size `size` that has random bytes.
/// 
///The RNG is specified using the flags.
pub fn generate_random_bytes(
    size: usize,
    flags: u32
) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size);

    unsafe {
        let mut bytes_written = 0;
        buf.set_len(size);

        while bytes_written < size {
            let ret = getrandom(
                buf.as_mut_ptr() as *mut c_void,
                size - bytes_written,
                flags
            );

            if ret >= 0 {
                bytes_written += ret as usize;
            }
        }
    }

    return buf;
}

///Returns a vector of size `element_count` that has random values
///of type `T`. (T is mostly a numeric type)
/// 
///The RNG is specified using the flags.
pub fn generate_random<T>(
    element_count: usize,
    flags: u32
) -> Vec<T> where T: Copy + Add + Sub + Mul + Div +
    AddAssign + SubAssign + MulAssign + DivAssign +
    PartialOrd + PartialEq {
    let mut buf = Vec::with_capacity(
        size_of::<T>() * element_count
    );

    unsafe {
        let mut bytes_written = 0;
        buf.set_len(element_count);

        while bytes_written < size_of::<T>() * element_count {
            let ret = getrandom(
                buf.as_mut_ptr() as *mut c_void,
                size_of::<T>() * element_count - bytes_written,
                flags
            );

            if ret > 0 {
                bytes_written += ret as usize;
            }
        }
    }

    return buf;
}

///Fills a buffer with `element_count` random values
///of type `T`. (T is mostly a numeric type)
/// 
///The RNG is specified using the flags.
/// 
///SAFETY: `buf_ptr` should be a valid pointer.
pub unsafe fn fill_random<T>(
    buf_ptr: *mut c_void,
    element_count: usize,
    flags: u32
) where T: Copy + Add + Sub + Mul + Div +
    AddAssign + SubAssign + MulAssign + DivAssign +
    PartialOrd + PartialEq {
    unsafe {
        let mut bytes_written = 0;

        while bytes_written < size_of::<T>() * element_count {
            let ret = getrandom(
                buf_ptr,
                size_of::<T>() * element_count - bytes_written,
                flags
            );

            if ret > 0 {
                bytes_written += ret as usize;
            }
        }
    }
}