extern crate libc;

pub type USHORT = u16;
pub type WORD = USHORT;
pub type ATOM = WORD;

pub type INT = i32;
pub type INT_PTR = *mut INT;
pub type BOOL = INT;

pub type UINT = u32;
pub type UINT_PTR = *mut UINT;
pub type DWORD = UINT;
pub type WPARAM = UINT_PTR;

pub type LONG = i64;
pub type LONG_PTR = *mut LONG;
pub type LRESULT = LONG_PTR;
pub type LPARAM = LONG_PTR;

pub type ULONG = u64;
pub type QWORD = ULONG;

pub type LPCSTR = *const libc::c_char;

pub type LPVOID = *mut libc::c_void;
pub type HANDLE = LPVOID;
pub type HBRUSH = HANDLE;
pub type HCURSOR = HANDLE;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMENU = HANDLE;
pub type HWND = HANDLE;

pub type WNDPROC = extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;

/*
    ShowWindow()
*/
pub const SW_HIDE                   : INT           = 0;
pub const SW_SHOWNORMAL             : INT           = 1;
pub const SW_NORMAL                 : INT           = 1;
pub const SW_SHOWMINIMIZED          : INT           = 2;
pub const SW_SHOWMAXIMIZED          : INT           = 3;
pub const SW_MAXIMIZE               : INT           = 3;
pub const SW_SHOWNOACTIVATE         : INT           = 4;
pub const SW_SHOW                   : INT           = 5;
pub const SW_MINIMIZE               : INT           = 6;
pub const SW_SHOWMINNOACTIVE        : INT           = 7;
pub const SW_SHOWNA                 : INT           = 8;
pub const SW_RESTORE                : INT           = 9;
pub const SW_SHOWDEFAULT            : INT           = 10;
pub const SW_FORCEMINIMIZE          : INT           = 11;
pub const SW_MAX                    : INT           = 11;

/*
    Window Styles
*/
pub const WS_OVERLAPPED             : LONG          = 0x00000000;
pub const WS_POPUP                  : LONG          = 0x80000000;
pub const WS_CHILD                  : LONG          = 0x40000000;
pub const WS_MINIMIZE               : LONG          = 0x20000000;
pub const WS_VISIBLE                : LONG          = 0x10000000;
pub const WS_DISABLED               : LONG          = 0x08000000;
pub const WS_CLIPSIBLINGS           : LONG          = 0x04000000;
pub const WS_CLIPCHILDREN           : LONG          = 0x02000000;
pub const WS_MAXIMIZE               : LONG          = 0x01000000;
pub const WS_CAPTION                : LONG          = 0x00C00000;
pub const WS_BORDER                 : LONG          = 0x00800000;
pub const WS_DLGFRAME               : LONG          = 0x00400000;
pub const WS_VSCROLL                : LONG          = 0x00200000;
pub const WS_HSCROLL                : LONG          = 0x00100000;
pub const WS_SYSMENU                : LONG          = 0x00080000;
pub const WS_THICKFRAME             : LONG          = 0x00040000;
pub const WS_GROUP                  : LONG          = 0x00020000;
pub const WS_TABSTOP                : LONG          = 0x00010000;
pub const WS_MINIMIZEBOX            : LONG          = 0x00020000;
pub const WS_MAXIMIZEBOX            : LONG          = 0x00010000;
pub const WS_TILED                  : LONG          = WS_OVERLAPPED;
pub const WS_ICONIC                 : LONG          = WS_MINIMIZE;
pub const WS_SIZEBOX                : LONG          = WS_THICKFRAME;
pub const WS_TILEDWINDOW            : LONG          = WS_OVERLAPPEDWINDOW;
pub const WS_OVERLAPPEDWINDOW       : LONG          = (WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU |
                                                       WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX);
pub const WS_POPUPWINDOW            : LONG       = (WS_POPUP | WS_BORDER | WS_SYSMENU);
pub const WS_CHILDWINDOW            : LONG       = (WS_CHILD);
pub const WS_EX_DLGMODALFRAME       : LONG       = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY      : LONG       = 0x00000004;
pub const WS_EX_TOPMOST             : LONG       = 0x00000008;
pub const WS_EX_ACCEPTFILES         : LONG       = 0x00000010;
pub const WS_EX_TRANSPARENT         : LONG       = 0x00000020;
pub const WS_EX_MDICHILD            : LONG       = 0x00000040;
pub const WS_EX_TOOLWINDOW          : LONG       = 0x00000080;
pub const WS_EX_WINDOWEDGE          : LONG       = 0x00000100;
pub const WS_EX_CLIENTEDGE          : LONG       = 0x00000200;
pub const WS_EX_CONTEXTHELP         : LONG       = 0x00000400;
pub const WS_EX_RIGHT               : LONG       = 0x00001000;
pub const WS_EX_LEFT                : LONG       = 0x00000000;
pub const WS_EX_RTLREADING          : LONG       = 0x00002000;
pub const WS_EX_LTRREADING          : LONG       = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR       : LONG       = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR      : LONG       = 0x00000000;
pub const WS_EX_CONTROLPARENT       : LONG       = 0x00010000;
pub const WS_EX_STATICEDGE          : LONG       = 0x00020000;
pub const WS_EX_APPWINDOW           : LONG       = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW    : LONG       = (WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE);
pub const WS_EX_PALETTEWINDOW       : LONG       = (WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST);
pub const WS_EX_LAYERED             : LONG       = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT     : LONG       = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP : LONG       = 0x00200000;
pub const WS_EX_LAYOUTRTL           : LONG       = 0x00400000;
pub const WS_EX_COMPOSITED          : LONG       = 0x02000000;
pub const WS_EX_NOACTIVATE          : LONG       = 0x08000000;

pub struct WNDCLASSEXA {
	pub cbSize : UINT,
	pub style : UINT,
	pub lpfnWndProc : WNDPROC,
	pub cbClsExtra : INT,
	pub cbWndExtra : INT,
	pub hInstance : HINSTANCE,
	pub hIcon : HICON,
	pub hCursor : HCURSOR,
	pub hbrBackground : HBRUSH,
	pub lpszMenuName : LPCSTR,
	pub lpszClassName : LPCSTR,
	pub hIconSm : HICON
}

