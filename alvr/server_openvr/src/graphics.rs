static QUAD_SHADER_COMP_SPV: &[u8] = include_bytes!("../cpp/platform/linux/shader/quad.comp.spv");
static COLOR_SHADER_COMP_SPV: &[u8] = include_bytes!("../cpp/platform/linux/shader/color.comp.spv");
static FFR_SHADER_COMP_SPV: &[u8] = include_bytes!("../cpp/platform/linux/shader/ffr.comp.spv");
static RGBTOYUV420_SHADER_COMP_SPV: &[u8] =
    include_bytes!("../cpp/platform/linux/shader/rgbtoyuv420.comp.spv");

pub fn initialize_shaders() {
    unsafe {
        crate::QUAD_SHADER_COMP_SPV_PTR = QUAD_SHADER_COMP_SPV.as_ptr();
        crate::QUAD_SHADER_COMP_SPV_LEN = QUAD_SHADER_COMP_SPV.len() as _;
        crate::COLOR_SHADER_COMP_SPV_PTR = COLOR_SHADER_COMP_SPV.as_ptr();
        crate::COLOR_SHADER_COMP_SPV_LEN = COLOR_SHADER_COMP_SPV.len() as _;
        crate::FFR_SHADER_COMP_SPV_PTR = FFR_SHADER_COMP_SPV.as_ptr();
        crate::FFR_SHADER_COMP_SPV_LEN = FFR_SHADER_COMP_SPV.len() as _;
        crate::RGBTOYUV420_SHADER_COMP_SPV_PTR = RGBTOYUV420_SHADER_COMP_SPV.as_ptr();
        crate::RGBTOYUV420_SHADER_COMP_SPV_LEN = RGBTOYUV420_SHADER_COMP_SPV.len() as _;
    }
}
