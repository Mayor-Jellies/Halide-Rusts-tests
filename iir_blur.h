#ifndef HALIDE__bin___host___iir_blur_h
#define HALIDE__bin___host___iir_blur_h
#include <stdint.h>

// Forward declarations of the types used in the interface
// to the Halide pipeline.
//
// For the definitions of these structs, include HalideRuntime.h

// Halide's representation of a multi-dimensional array.
// Halide::Runtime::Buffer is a more user-friendly wrapper
// around this. Its declaration is in HalideBuffer.h
struct halide_buffer_t;

// Metadata describing the arguments to the generated function.
// Used to construct calls to the _argv version of the function.
struct halide_filter_metadata_t;

#ifndef HALIDE_MUST_USE_RESULT
#ifdef __has_attribute
#if __has_attribute(nodiscard)
#define HALIDE_MUST_USE_RESULT [[nodiscard]]
#elif __has_attribute(warn_unused_result)
#define HALIDE_MUST_USE_RESULT __attribute__((warn_unused_result))
#else
#define HALIDE_MUST_USE_RESULT
#endif
#else
#define HALIDE_MUST_USE_RESULT
#endif
#endif

#ifndef HALIDE_FUNCTION_ATTRS
#define HALIDE_FUNCTION_ATTRS
#endif



#ifdef __cplusplus
extern "C" {
#endif

HALIDE_FUNCTION_ATTRS
int iir_blur_par_for_transpose_s0_v2_rebased_par_for_transpose_s0_v1_v6(void *__user_context, int32_t _transpose_s0_v1_v6, uint8_t *_closure_arg__1);
HALIDE_FUNCTION_ATTRS
int iir_blur_par_for_transpose_s0_v2_rebased(void *__user_context, int32_t _transpose_s0_v2_rebased, uint8_t *_closure_arg);
HALIDE_FUNCTION_ATTRS
int iir_blur_par_for_transpose__1_s0_v2_rebased_par_for_transpose__1_s0_v1_v13(void *__user_context, int32_t _transpose__1_s0_v1_v13, uint8_t *_closure_arg__3);
HALIDE_FUNCTION_ATTRS
int iir_blur_par_for_transpose__1_s0_v2_rebased(void *__user_context, int32_t _transpose__1_s0_v2_rebased, uint8_t *_closure_arg__2);
HALIDE_FUNCTION_ATTRS
int iir_blur(struct halide_buffer_t *_input_buffer, float _alpha, struct halide_buffer_t *_transpose__1_buffer);

HALIDE_FUNCTION_ATTRS
int iir_blur_argv(void **args);

HALIDE_FUNCTION_ATTRS
const struct halide_filter_metadata_t *iir_blur_metadata();

#ifdef __cplusplus
}  // extern "C"
#endif

#endif
