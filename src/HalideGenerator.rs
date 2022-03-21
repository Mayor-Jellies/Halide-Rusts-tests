use crate::halide_buffer_t;

#[link(name="iir_blur", kind="static")]
extern "C" {
    pub fn iir_blur(
        src: halide_buffer_t,
        float: f32,
        dst: halide_buffer_t,
    );
}
