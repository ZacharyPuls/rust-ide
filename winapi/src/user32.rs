
extern "stdcall" {
	pub fn RegisterClassExA(
			lpWndClass : *mut WNDCLASSEXA
	) -> ATOM;

	pub fn CreateWindowExA(
		dwExStyle : u64,
		lpClassName : *const libc::c_char,
		lpWindowName : *const libc::c_char,
		dwStyle : u64,
		X : i32,
		Y : i32,
		nWidth : i32,
		nHeight : i32,
		hWndParent : types::HWND,
		hMenu : types::HMENU,
		hInstance : types::HINSTANCE,
		lpParam : types::LPVOID
	) -> i32;
}