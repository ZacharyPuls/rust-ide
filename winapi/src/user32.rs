extern crate libc;

use ::types::*;

#[link(name = "user32")]
extern "stdcall" {
	pub fn RegisterClassExA(
			lpWndClass : *mut WNDCLASSEXA
	) -> ATOM;

	pub fn CreateWindowExA(
		dwExStyle: DWORD,
		lpClassName: LPCSTR,
		lpWindowName: LPCSTR,
		dwStyle: DWORD,
		X: i32,
		Y: i32,
		nWidth: i32,
		nHeight: i32,
		hWndParent: HWND,
		hMenu: HMENU,
		hInstance: HINSTANCE,
		lpParam: LPVOID
	) -> HWND;

	pub fn ShowWindow(
	    hWnd: HWND,
	    nCmdShow: i32
	) -> BOOL;

	pub fn DefWindowProcA(
	    hWnd: HWND,
	    uMsg: UINT,
	    wParam: WPARAM,
	    lParam: LPARAM
	) -> LRESULT;
}