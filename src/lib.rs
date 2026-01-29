use std::{fs::OpenOptions, io::Read};

pub mod ffi;

pub fn read_shader(path: &str) -> Vec<u8> {
    let mut bytes = Vec::new();

    let mut shader = OpenOptions::new()
        .read(true).open(path).unwrap();

    shader.read_to_end(&mut bytes).unwrap();

    bytes
}