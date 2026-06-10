#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{ffi, os::raw, ptr};

/*****
* 4096-aligned buffer to use for data transfers.
* Sadly align() has to be hardcoded so I settled for the standard 4 KiB,
* modify that number if you need bigger alignement for whatever reason.
* Instantiate on the heap with:
* let mut buf = Box::new(PageAlignedBuffer<{type}, {size}>(0, {size})) 
******/
#[repr(align(4096))]
pub struct PageAlignedBuffer<T, const N: usize>(pub [T;N]);

/*****
 * Returns a void* to a PageAlignedBuffer.
 * If the buffer is in a box (heap allocatad), you need to dereference it.
 *****/
pub fn get_buf_raw_ptr<T, const N: usize>(buf: &mut PageAlignedBuffer<T, N>) -> *mut raw::c_void {
	buf as *const PageAlignedBuffer<T, N> as *mut raw::c_void
}

/*****
 * Reads the specified card's error string, and returns it as Some(String) if
 * it's not null (else None).
 * I initially tried to get it to return Some(&str), but the conversion required
 * UTF-8 while the SDK returns ASCII. At least the heap allocation only happens
 * when an error occurs (at which point you'd probably want to kill the program
 * anyway).
 *****/
pub fn get_error(h_device: drv_handle) -> Option<String> {
	let mut err_text_buf = [0u8; ERRORTEXTLEN as usize];
	let err_text_ptr: *mut i8 = err_text_buf.as_mut_ptr() as *mut ffi::c_char;
	if unsafe{spcm_dwGetErrorInfo_i64(
				h_device, ptr::null_mut(), ptr::null_mut(), err_text_ptr)} != 0 {
		return Some(unsafe{ffi::CStr::from_ptr(err_text_ptr)}.to_string_lossy().into_owned());
	}
	
	None
}