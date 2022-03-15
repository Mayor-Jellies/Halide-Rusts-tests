#![allow(warnings)]

pub mod HalideRuntime;

use std::ffi::c_void;
use std::ffi::*;
use std::os::raw::c_int;
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
    
include!("test.rs");

fn main(){

    println!("halide mainish thing");
    
    let img = Reader::open("cat.png").unwrap().decode().unwrap().to_rgb8();
    let (width, height) = (img.width(), img.height());
    let img_byte_vec = img.into_raw();
    
    let buf = halide_buffer(width as i32, height as i32, 10, Type::new(Kind::UInt, 8) , img_byte_vec.as_ptr() as *mut u8);
    
    save_buffer_with_format("myimg.jpg", &img_byte_vec, width, height, image::ColorType::Rgb8, image::ImageFormat::Jpeg).unwrap();
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Kind {
    Int = halide_type_code_t_halide_type_int as u8,
    UInt = halide_type_code_t_halide_type_uint as u8,
    Float = halide_type_code_t_halide_type_float as u8,
}


/// Type is used to define the type of pixel data in terms of kind and bits
/// For example, Type::new(Kind::UInt, 8) uses one 8-bit unsigned integer per channel
/// and Type::new(Kind::Float, 32) uses a float per channel, etc...
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Type(pub Kind, pub u8, pub u16);
impl Type {
    pub fn new(kind: Kind, bits: u8) -> Type {
        Type(kind, bits, 1)
    }

    pub fn new_with_lanes(kind: Kind, bits: u8, lanes: u16) -> Type {
        Type(kind, bits, lanes)
    }

    pub fn bits(&self) -> u8 {
        return self.1;
    }

    pub fn kind(&self) -> Kind {
        return self.0;
    }

    pub fn size(&self) -> usize {
        self.bits() as usize / 8
    }
}

fn halide_buffer(
    width: i32,
    height: i32,
    channels: i32,
    t: Type,
    data: *mut u8,
) -> halide_buffer_t {
    let t = halide_type_t {
        code: t.0 as u8,
        bits: t.1,
        lanes: t.2,
    };

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
        dimensions: if channels < 2 { 2 } else { 3 },
        host: data,
        flags: 0,
        padding: std::ptr::null_mut(),
        type_: t,
        dim: dim.as_mut_ptr(),
    };

    std::mem::forget(dim);

    buf
}

