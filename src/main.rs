#![allow(warnings)]

pub mod HalideRuntime;
pub mod HalideGenerator;

use std::ffi::c_void;
use std::ffi::*;
use std::os::raw::c_int;
use std::sync::mpsc::channel;
use crate::HalideRuntime::{halide_malloc, halide_profiler_report};
use crate::HalideRuntime::halide_free;

//////////////////////////////////////////////////////////////////////////////////////////////////
use crate::HalideRuntime::halide_type_code_t;
use crate::HalideRuntime::halide_type_t;
use crate::HalideRuntime::halide_buffer_t;
use crate::HalideRuntime::halide_dimension_t;
use crate::HalideRuntime::halide_type_code_t_halide_type_int;
use crate::HalideRuntime::halide_type_code_t_halide_type_uint;
use crate::HalideRuntime::halide_type_code_t_halide_type_float;
//////////////////////////////////////////////////////////////////////////////////////////////////

use image::io::Reader;
use image::save_buffer_with_format;
use crate::HalideGenerator::iir_blur;

include!("test.rs");

fn main(){

    println!("halide mainish thing");
    
    let img = Reader::open("cat.png").unwrap().decode().unwrap().to_rgb32f();
    let img2 = Reader::open("cat.png").unwrap().decode().unwrap().to_rgb32f();

    let (width, height) = (img.width(), img.height());
    let mut img_byte_vec = img.into_raw();
    let mut img_byte_vec2: Vec<f32> = vec![0.0; ((width as i32 )* (height as i32) ) as usize];
    
    let mut inbuf: halide_buffer_t = halide_buffer(width as i32, height as i32, 1, halide_type_t{bits: 32,code: 2,lanes: 1}, img_byte_vec.as_mut_ptr());
    println!("{:?}",img_byte_vec.as_ptr() );
    let mut outbuf:  halide_buffer_t = halide_buffer(width as i32,height as i32, 1, halide_type_t{bits: 32,code: 2,lanes: 1}, img_byte_vec2.as_mut_ptr());

    println!("{:?}",img_byte_vec2.as_ptr() );


    unsafe {
         iir_blur(&mut inbuf , 10000.0, &mut outbuf);
    }

   // for i in 0.. img_byte_vec.len()-1{
     //   assert_ne!(img_byte_vec[i],img_byte_vec2[i],"{}",i);
    //}
    //save
//save_buffer_with_format("myimg.png", &img_byte_vec2, width, height, image::ColorType::Rgb32F, image::ImageFormat::Png).unwrap();
}


fn halide_buffer(
    width: i32,
    height: i32,
    channels: i32,
    t: halide_type_t,
    data: *mut f32,
) -> halide_buffer_t {


    let mut dim = Vec::new();

    dim.push(halide_dimension_t {
        flags: 0,
        min: 0,
        extent: width,
        stride: channels,
    });

    dim.push(halide_dimension_t {
        flags: 0,
        min: 0,
        extent: height,
        //stride: channels,
        stride: channels * width,
    });

    if channels > 1 {
        dim.push(halide_dimension_t {
            flags: 0,
            min: 0,
            extent: channels,
            stride: 1,
        });
    }

    dim.shrink_to_fit();

    let buf = halide_buffer_t {
        device: 0,
        device_interface: std::ptr::null(),
        dimensions: 3,
        host: data,
        flags: 0,
        padding: std::ptr::null_mut(),
        type_: t,
        dim: dim.as_mut_ptr(),
    };

    std::mem::forget(dim);

    buf
}

