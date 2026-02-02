use crate::ffi::{
    glGetIntegeri_v,
    glGetIntegerv
};

unsafe extern "C" {
    fn create_shader_from_src(
        src_code: *const u8,
        len: i32
    ) -> u32;

    ///Compiles the shader created using `create_shader`
    /// 
    ///Program exits if compilation fails
    pub fn compile_shader(
        shader: u32
    );

    ///Creates a single-shader program
    /// 
    ///Uses the shader created using `create_shader`
    pub fn create_program(
        shader: u32
    ) -> u32;

    ///Destroys the GLES shader and program
    /// 
    ///#### USE IT ONLY ONCE, AT THE END OF THE PROGRAM!
    pub fn gles_cleanup(
        program: u32,
        shader: u32
    );
}

///Creates a shader from a single ESSL source file
pub fn create_shader(src_code: Vec<u8>) -> u32 {
    unsafe {
        create_shader_from_src(src_code.as_ptr(), src_code.len() as i32)
    }
}

const GL_MAX_COMPUTE_WORK_GROUP_COUNT: u32 = 0x91BE;
const GL_MAX_COMPUTE_WORK_GROUP_SIZE: u32 = 0x91BF;
const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: u32 = 0x90EB;

///Gives max. no. of workgroups that may be dispatched
///by a single dispatch command (per dimension)
pub fn get_max_work_group_count() -> (i32, i32, i32) {
    let mut result = [0; 3];

    unsafe {
        glGetIntegeri_v(
            GL_MAX_COMPUTE_WORK_GROUP_COUNT,
            0,
            &raw mut result[0]
        );
        glGetIntegeri_v(
            GL_MAX_COMPUTE_WORK_GROUP_COUNT,
            1,
            &raw mut result[1]
        );
        glGetIntegeri_v(
            GL_MAX_COMPUTE_WORK_GROUP_COUNT,
            2,
            &raw mut result[2]
        );
    }

    (result[0], result[1], result[2])
}

///Gives max. local size of a compute workgroup (per dimension)
pub fn get_max_work_group_size() -> (i32, i32, i32) {
    let mut result = [0; 3];

    unsafe {
        glGetIntegeri_v(
            GL_MAX_COMPUTE_WORK_GROUP_SIZE,
            0,
            &raw mut result[0]
        );
        glGetIntegeri_v(
            GL_MAX_COMPUTE_WORK_GROUP_SIZE,
            1,
            &raw mut result[1]
        );
        glGetIntegeri_v(
            GL_MAX_COMPUTE_WORK_GROUP_SIZE,
            2,
            &raw mut result[2]
        );
    }

    (result[0], result[1], result[2])
}

///Gives max. total compute shader invocations
///in a single local work-group
pub fn get_max_work_group_invocations() -> i32 {
    let mut result = 0;

    unsafe {
        glGetIntegerv(
            GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS,
            &raw mut result
        );
    }

    result
}