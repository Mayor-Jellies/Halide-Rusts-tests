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
    
    let img = Reader::open("catSmol.jpg").unwrap().decode().unwrap().to_rgb8();
    
      //  let img = Reader::open("cat.png").unwrap().decode().unwrap().to_rgb32f();

    let (width, height) = (img.width(), img.height());
    let mut img_byte_vec = img.into_raw();

    let mut input: Vec<f32> = vec![0.0;img_byte_vec.len()];
    for x in 0..img_byte_vec.len(){
        input[x] = img_byte_vec[x] as f32;
    }




    let mut img_byte_vec2: Vec<f32> = vec![0.0; ((width as i32 )* (height as i32) * 3) as usize];


    

    
    println!("{}", img_byte_vec[1000]);
    println!("{}", img_byte_vec2[1000]);
    
    let mut inbuf: halide_buffer_t = halide_buffer(width as i32, height as i32, 1, halide_type_t{bits: 32,code: 2,lanes: 1}, input.as_mut_ptr(), 1);

    let mut outbuf:  halide_buffer_t = halide_buffer(width as i32,height as i32, 1, halide_type_t{bits: 32,code: 2,lanes: 1}, img_byte_vec2.as_mut_ptr(), 0);




	// Trying to make smaller vectors to test with
    /*
    let mut testvec = vec![1.0f32,2.0,3.0];
    let mut testvec2 = vec![0.0; 3 as usize];
    */
	//let mut testin: halide_buffer_t = halide_buffer(3 as i32, 1 as i32, 1, halide_type_t{bits: 32,code: 2,lanes: 1}, testvec.as_mut_ptr());
    //println!("{:?}",img_byte_vec.as_ptr() );
	//let mut testout:  halide_buffer_t = halide_buffer(3 as i32,1 as i32, 1, halide_type_t{bits: 32,code: 2,lanes: 1}, testvec2.as_mut_ptr());
    
    
    
    unsafe {
         iir_blur(&mut inbuf, 0.5, &mut outbuf);
    }

    println!("{}", img_byte_vec[1000]);
    println!("{}", img_byte_vec2[1000]);
   // for i in 0.. img_byte_vec.len()-1{
     //   assert_ne!(img_byte_vec[i],img_byte_vec2[i],"{}",i);
    //}
    //save
    let mut output: Vec<u8> = vec![0;img_byte_vec2.len()];
    for x in 0..img_byte_vec2.len(){
        output[x] = img_byte_vec2[x] as u8;
    }

	//let output: Vec<u8> =

	save_buffer_with_format("myimg.jpg", &output, width, height, image::ColorType::Rgb8, image::ImageFormat::Jpeg).unwrap();
	
	//save_buffer_with_format("myimg.png", &img, width, height, image::ColorType::Rgb32F, image::ImageFormat::Png).unwrap();
	
	//newimg.save("myimg.png", image::ImageFormat::Png).unwrap();
}


fn halide_buffer(
    width: i32,
    height: i32,
    channels: i32,
    t: halide_type_t,
    data: *mut f32,
    flags: u64,
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

	dim.push(halide_dimension_t {
	    flags: 0,
	    min: 0,
	    extent: 3,
	    stride: channels * width * height,
	});
    
    dim.shrink_to_fit();

    let buf = halide_buffer_t {
        device: 0,
        device_interface: std::ptr::null(),
        dimensions: 3,
        host: data,
        flags: flags,
        padding: std::ptr::null_mut(),
        type_: t,
        dim: dim.as_mut_ptr(),
    };

    std::mem::forget(dim);

    buf
}

