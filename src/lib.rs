use std::{fs::OpenOptions, io::Read, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}, os::raw::c_void};

use crate::ffi::getrandom;

pub mod ffi;

pub fn read_shader(path: &str) -> Vec<u8> {
    let mut bytes = Vec::new();

    let mut shader = OpenOptions::new()
        .read(true).open(path).unwrap();

    shader.read_to_end(&mut bytes).unwrap();

    bytes
}

pub const GRND_INSECURE: u32 = 0x4;
pub const GRND_RANDOM: u32 = 0x2;
pub const GRND_URANDOM: u32 = 0x0;
pub const GRND_NONBLOCK: u32 = 0x1;

pub fn generate_random_bytes(
    element_count: usize,
    flags: u32
) -> Vec<u8> {
    let mut buf = Vec::with_capacity(element_count);

    unsafe {
        let mut bytes_written = 0;
        buf.set_len(element_count);

        while bytes_written < element_count {
            let ret = getrandom(
                buf.as_mut_ptr() as *mut c_void,
                element_count - bytes_written,
                flags
            );

            if ret >= 0 {
                bytes_written += ret as usize;
            }
        }
    }

    return buf;
}

pub fn generate_random<T>(
    element_count: usize,
    flags: u32
) -> Vec<T> where T: Copy + Add + Sub + Mul + Div +
    AddAssign + SubAssign + MulAssign + DivAssign + PartialOrd + PartialEq {
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