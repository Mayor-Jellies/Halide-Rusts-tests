use crate::halide_buffer_t;

#[link(name="iir_blur", kind="static")]
extern "C" {
    pub fn iir_blur(
        in_buf: *mut halide_buffer_t,
        alpha: f32,
        out_buf: *mut halide_buffer_t,
    ) -> ::std::os::raw::c_int;
}