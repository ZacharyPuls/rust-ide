use std;

use libc;

use winapi::types::*;
use winapi::messages::*;
use winapi::user32::*;
use winapi::kernel32::*;

use std::ffi::CString;

const DEFAULT_WINDOW_CLASS_STYLE: DWORD = CS_HREDRAW | CS_VREDRAW;
const DEFAULT_WINDOW_STYLE: DWORD = WS_VISIBLE;
const DEFAULT_WINDOW_EX_STYLE: DWORD = WS_EX_APPWINDOW;

fn display_func() {
}

extern "system" fn wnd_proc(h_wnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match u_msg {
        WM_CLOSE => std::process::exit(0),
        _ => unsafe { DefWindowProcA(h_wnd, u_msg, w_param, l_param) },
    }
}

/* TODO: Allow for registering of callbacks instead of the hard-coded ones.
   TODO: Most likely this can be done with std::boxed::FnBox<>
pub fn register_display_callback(func: Box<Fn() -> ()>) {
    display_func = func;
}
*/

pub unsafe fn create_window(title: &'static str, x: i32, y: i32, width: i32, height: i32, background_color: HBRUSH)  {

    println!("Creating window with title {}", title);

    let current_hinstance: HINSTANCE = GetModuleHandleA("") as HINSTANCE;

    let wnd_class_ex: *mut WNDCLASSEXA = &mut WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as UINT,
        style: DEFAULT_WINDOW_CLASS_STYLE as UINT,
        lpfnWndProc: wnd_proc as WNDPROC,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: current_hinstance,
        hIcon: 0 as HICON,
        hCursor: 0 as HCURSOR,
        hbrBackground: background_color,
        lpszMenuName: "",
        lpszClassName: title,
        hIconSm: 0 as HICON
    };

    RegisterClassExA(wnd_class_ex);

    let h_wnd: HWND = CreateWindowExA(
        DEFAULT_WINDOW_EX_STYLE as DWORD,
        title,
        title,
        DEFAULT_WINDOW_STYLE,
        x,
        y,
        width,
        height,
        0 as HWND,
        0 as HMENU,
        current_hinstance,
        0 as LPVOID
    );

    let mut msg: MSG = MSG {
        hWnd: 0 as HWND,
        message: 0 as UINT,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT {
            x: 0,
            y: 0
        }
    };

    let lp_msg = &mut msg as LPMSG;

    while GetMessageA(lp_msg, h_wnd, 0 as UINT, 0 as UINT) != 0 {
        TranslateMessage(lp_msg);
        DispatchMessageA(lp_msg);

        display_func();
    }
}