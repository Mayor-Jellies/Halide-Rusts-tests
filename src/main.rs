#![allow(warnings)]


pub mod HalideRuntime;

use std::ffi::c_void;
use std::ffi::*;
use std::os::raw::c_int;
use crate::HalideRuntime::{halide_malloc, halide_profiler_report};
use crate::HalideRuntime::halide_free;

include!("test.rs");

fn main(){

    println!("halide mainish thing");

    let raw_ptr: *mut ::std::os::raw::c_void = std::ptr::null_mut();
    //let raw_ptr = 10;
    use crate::HalideRuntime::halide_malloc;
    unsafe {
       halide_profiler_report(raw_ptr);
    }

}
