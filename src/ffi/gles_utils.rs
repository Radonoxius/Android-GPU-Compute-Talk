unsafe extern "C" {
    fn create_shader_from_src(
        src_code: *const u8,
        len: i32
    ) -> u32;

    pub fn compile_shader(
        shader: u32
    );

    pub fn create_program(
        shader: u32
    ) -> u32;

    pub fn gles_cleanup(
        program: u32,
        shader: u32
    );
}

pub unsafe fn create_shader(src_code: Vec<u8>) -> u32 {
    unsafe {
        create_shader_from_src(src_code.as_ptr(), src_code.len() as i32)
    }
}
