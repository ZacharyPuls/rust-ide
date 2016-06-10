extern crate libc;
extern crate winapi;

use std::ffi::CString;

use winapi::types::*;
use winapi::messages::*;
use winapi::user32::*;

pub const VERSION: &'static str = "0.1.0";

extern "system" fn wnd_proc(hWnd: HWND, uMsg: UINT, wParam: WPARAM,lParam:
LPARAM) ->
 LRESULT {
    match uMsg {
        WM_CLOSE => std::process::exit(0),
        _ => unsafe { DefWindowProcA(hWnd, uMsg, wParam, lParam) },
    }

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
	        WS_EX_APPWINDOW,
            lpszClassName,
            CString::new("Rust IDE").unwrap().as_ptr(),
            WS_OVERLAPPEDWINDOW,
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

        let mut msg: MSG = MSG { hWnd: 0 as HWND, message: 0 as UINT, wParam: 0 as WPARAM, lParam: 0 as LPARAM, time: 0 as DWORD, pt: POINT{x: 0,y: 0} };
        let lpMsg = &mut msg as *mut MSG;

        while GetMessageA(lpMsg, hWnd, 0 as UINT, 0 as UINT) != 0 {
            TranslateMessage(lpMsg);
            DispatchMessageA(lpMsg);
        }
     }
}
