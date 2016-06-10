extern crate libc;

use ::types::*;

use std::string::String;

#[link(name = "user32")]
extern "stdcall" {
	pub fn RegisterClassExW(
			lpWndClass : *mut WNDCLASSEXW
	) -> ATOM;

	pub fn CreateWindowExW(
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

	pub fn DefWindowProcW(
	    hWnd: HWND,
	    uMsg: UINT,
	    wParam: WPARAM,
	    lParam: LPARAM
	) -> LRESULT;

	pub fn GetMessageW(
	    lpMsg: LPMSG,
	    hWnd: HWND,
	    wMsgFilterMin: UINT,
	    wMsgFilterMax: UINT
	) -> BOOL;

	pub fn TranslateMessage(
	    lpMsg: LPMSG
	) -> BOOL;

	pub fn DispatchMessageW(
	    lpMsg: LPMSG
	) -> LRESULT;
}