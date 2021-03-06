extern crate libc;

use ::types::*;

use std::string::String;

#[link(name = "user32")]
extern "stdcall" {
	pub fn RegisterClassExA(
			lpWndClass : *mut WNDCLASSEXA
	) -> ATOM;

	pub fn CreateWindowExA(
		dwExStyle: DWORD,
		lpClassName: &str,
		lpWindowName: &str,
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

	pub fn GetMessageA(
	    lpMsg: LPMSG,
	    hWnd: HWND,
	    wMsgFilterMin: UINT,
	    wMsgFilterMax: UINT
	) -> BOOL;

	pub fn TranslateMessage(
	    lpMsg: LPMSG
	) -> BOOL;

	pub fn DispatchMessageA(
	    lpMsg: LPMSG
	) -> LRESULT;
}