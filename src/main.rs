extern crate libc;

use std::ffi::CString;

pub type HWND = *mut libc::c_void;

pub const VERSION: &'static str = "0.1.0";

#[link_args = "-Wl,--subsystem,windows"]
fn main() {
    println!("RustIDE Version {}", VERSION);
	unsafe {
	CreateWindowExA(0x00040000, 
	CString::new("testWindowClass").unwrap().as_ptr(), 
	CString::new("Rust IDE").unwrap().as_ptr(), 
	0x00000000,
	0x80000000,
	0x80000000,
	640, 
	480,
	0 as HWND, 
	0 as HWND, 
	0 as HWND, 
	0 as HWND);
	}
}
