static mut IS_FN_CACHED: bool = false;
static mut IS_SUPPORTED: bool = false;


extern "C" fn _dummy(_c: u32) {}

#[allow(non_upper_case_globals)]
static mut _glMaxActiveShaderCoresARM: unsafe extern "C" fn(u32) = _dummy;

unsafe extern "C" {
    safe fn _setup_glMaxActiveShaderCoresARM() -> unsafe extern "C" fn(u32);

    safe fn _is_supported_ARM_core_properties() -> bool;
}

unsafe extern "C" {
    fn glGetIntegerv(pname: u32, data: *mut i32);

    fn glGetInteger64v(pname: u32, data: *mut i64);
}

pub enum CorePropertiesARM {
    CoreCount,
    ActiveCoreCount,
    PresentMask,
    MaxWarpCount,
    PixelRate,
    TexelRate,
    FMARate
}

impl Into<u32> for CorePropertiesARM {
    fn into(self) -> u32 {
        match self {
            Self::CoreCount => 0x96F0,
            Self::ActiveCoreCount => 0x96F1,

            Self::PresentMask => 0x96F2,

            Self::MaxWarpCount => 0x96F3,
            Self::PixelRate => 0x96F4,
            Self::TexelRate => 0x96F5,
            Self::FMARate => 0x96F6
        }
    }
}

///Sets the maximum number of GPU cores
///that can be used by the ARM GPU.
///
///SAFETY: `count` should be positive and less than or
///equal to the maximum core count
///
///This function does nothing on other platforms
#[allow(non_snake_case)]
#[allow(useless_ptr_null_checks)]
pub unsafe fn glMaxActiveShaderCoresARM(count: u32) {
    unsafe {
        if IS_FN_CACHED {
            _glMaxActiveShaderCoresARM(count);
        }
        else {
            if _is_supported_ARM_core_properties() != true {
                return;
            }
            else {
                let f_ptr = _setup_glMaxActiveShaderCoresARM();

                IS_FN_CACHED = true;
                IS_SUPPORTED = true;
                _glMaxActiveShaderCoresARM = f_ptr;

                _glMaxActiveShaderCoresARM(count);
            }
        }
    }
}

///Get specified ARM GPU Core
///properties.
/// 
///-1 is returned on error or if
///platform is NOT SUPPORTED
/// 
///This function does nothing on other platforms
#[allow(non_snake_case)]
pub fn get_core_properties_ARM(property: CorePropertiesARM) -> i64 {
    unsafe {
        let pname: u32 = property.into();
        let mut result: i64 = -1;

        if IS_SUPPORTED {
            if pname == 0x96F2 {
                glGetInteger64v(pname, &raw mut result);
            }
            else {
                let mut _res: i32 = 0;
                glGetIntegerv(pname, &raw mut _res);

                result = _res as i64;
            }
        }
        else {
            if _is_supported_ARM_core_properties() != true {
                return -1;
            }
            else {
                IS_SUPPORTED = true;

                if pname == 0x96F2 {
                    glGetInteger64v(pname, &raw mut result);
                }
                else {
                    let mut _res: i32 = 0;
                    glGetIntegerv(pname, &raw mut _res);

                    result = _res as i64;
                }
            }
        }

        result
    }
}