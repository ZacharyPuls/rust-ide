pub type USHORT = u16;
pub type WORD = USHORT;
pub type ATOM = WORD;

pub type INT = i32;
pub type INT_PTR = *mut INT;

pub type UINT = u32;
pub type UINT_PTR = *mut UINT;
pub type WPARAM = UINT_PTR;

pub type LONG = i64;
pub type LONG_PTR = *mut LONG;
pub type LRESULT = LONG_PTR;

pub type LPCSTR = *const libc::c_char;

pub type LPVOID = *mut libc::c_void;
pub type HANDLE = LPVOID;
pub type HBRUSH = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMENU = HANDLE;
pub type HWND = HANDLE;

pub type WNDPROC = fn(HWND, UINT, WPARAM, LPARAM);

pub struct WNDCLASSEXA {
	cbSize : UINT,
	style : UINT,
	lpfnWndProc : WNDPROC,
	cbClsExtra : INT,
	cbWndExtra : INT,
	hInstance : HINSTANCE,
	hIcon : HICON,
	hCursor : HCURSOR,
	hbrBackground : HBRUSH,
	lpszMenuName : LPCSTR,
	lpszClassName : LPCSTR,
	hIconSm : HICON
}

