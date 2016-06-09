extern crate libc;
extern crate winapi;

use std::ffi::CString;

use winapi::types::*;
use winapi::user32::*;

pub const VERSION: &'static str = "0.1.0";

extern "system" fn wnd_proc(hWnd: HWND, uMsg: UINT, wParam: WPARAM,lParam:
LPARAM) ->
 LRESULT {
    unsafe { DefWindowProcA(hWnd, uMsg, wParam, lParam) }
}

#[link_args = "-Wl,--subsystem,windows"]
fn main() {
    println!("RustIDE Version {}", VERSION);

	unsafe {

	    let lpszClassName = CString::new("rustIDE-WNDCLASSEX").unwrap().as_ptr();

	    let wndClassEx : *mut WNDCLASSEXA = &mut WNDCLASSEXA{
	        cbSize: std::mem::size_of::<WNDCLASSEXA>() as UINT,
	        style: 0x0002 | 0x0001,
	        lpfnWndProc: wnd_proc as WNDPROC,
	        cbClsExtra: 0,
	        cbWndExtra: 0,
	        hInstance: 0 as HINSTANCE,
	        hIcon: 0 as HICON,
	        hCursor: 0 as HCURSOR,
	        hbrBackground: 5 as HBRUSH,
	        lpszMenuName: CString::new("").unwrap().as_ptr(),
	        lpszClassName: lpszClassName,
	        hIconSm: 0 as HICON
	    };

	    RegisterClassExA(wndClassEx);

	    let hWnd : HWND = CreateWindowExA(
	        0x00040000,
            lpszClassName,
            CString::new("Rust IDE").unwrap().as_ptr(),
            0x00000000,
            0x80000000,
            0x80000000,
            640,
            480,
            0 as HWND,
            0 as HMENU,
            0 as HINSTANCE,
            0 as LPVOID
        );

        ShowWindow(hWnd, SW_SHOW);

        loop{}
     }
}
