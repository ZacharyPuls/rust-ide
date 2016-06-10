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
pub const SW_HIDE                   : INT            = 0;
pub const SW_SHOWNORMAL             : INT            = 1;
pub const SW_NORMAL                 : INT            = 1;
pub const SW_SHOWMINIMIZED          : INT            = 2;
pub const SW_SHOWMAXIMIZED          : INT            = 3;
pub const SW_MAXIMIZE               : INT            = 3;
pub const SW_SHOWNOACTIVATE         : INT            = 4;
pub const SW_SHOW                   : INT            = 5;
pub const SW_MINIMIZE               : INT            = 6;
pub const SW_SHOWMINNOACTIVE        : INT            = 7;
pub const SW_SHOWNA                 : INT            = 8;
pub const SW_RESTORE                : INT            = 9;
pub const SW_SHOWDEFAULT            : INT            = 10;
pub const SW_FORCEMINIMIZE          : INT            = 11;
pub const SW_MAX                    : INT            = 11;

/*
    Window Styles
*/
pub const WS_OVERLAPPED             : UINT           = 0x00000000;
pub const WS_POPUP                  : UINT           = 0x80000000;
pub const WS_CHILD                  : UINT           = 0x40000000;
pub const WS_MINIMIZE               : UINT           = 0x20000000;
pub const WS_VISIBLE                : UINT           = 0x10000000;
pub const WS_DISABLED               : UINT           = 0x08000000;
pub const WS_CLIPSIBLINGS           : UINT           = 0x04000000;
pub const WS_CLIPCHILDREN           : UINT           = 0x02000000;
pub const WS_MAXIMIZE               : UINT           = 0x01000000;
pub const WS_CAPTION                : UINT           = 0x00C00000;
pub const WS_BORDER                 : UINT           = 0x00800000;
pub const WS_DLGFRAME               : UINT           = 0x00400000;
pub const WS_VSCROLL                : UINT           = 0x00200000;
pub const WS_HSCROLL                : UINT           = 0x00100000;
pub const WS_SYSMENU                : UINT           = 0x00080000;
pub const WS_THICKFRAME             : UINT           = 0x00040000;
pub const WS_GROUP                  : UINT           = 0x00020000;
pub const WS_TABSTOP                : UINT           = 0x00010000;
pub const WS_MINIMIZEBOX            : UINT           = 0x00020000;
pub const WS_MAXIMIZEBOX            : UINT           = 0x00010000;
pub const WS_TILED                  : UINT           = WS_OVERLAPPED;
pub const WS_ICONIC                 : UINT           = WS_MINIMIZE;
pub const WS_SIZEBOX                : UINT           = WS_THICKFRAME;
pub const WS_TILEDWINDOW            : UINT           = WS_OVERLAPPEDWINDOW;
pub const WS_OVERLAPPEDWINDOW       : UINT           = (WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU |
                                                       WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX);
pub const WS_POPUPWINDOW            : UINT        = (WS_POPUP | WS_BORDER | WS_SYSMENU);
pub const WS_CHILDWINDOW            : UINT        = (WS_CHILD);
pub const WS_EX_DLGMODALFRAME       : UINT        = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY      : UINT        = 0x00000004;
pub const WS_EX_TOPMOST             : UINT        = 0x00000008;
pub const WS_EX_ACCEPTFILES         : UINT        = 0x00000010;
pub const WS_EX_TRANSPARENT         : UINT        = 0x00000020;
pub const WS_EX_MDICHILD            : UINT        = 0x00000040;
pub const WS_EX_TOOLWINDOW          : UINT        = 0x00000080;
pub const WS_EX_WINDOWEDGE          : UINT        = 0x00000100;
pub const WS_EX_CLIENTEDGE          : UINT        = 0x00000200;
pub const WS_EX_CONTEXTHELP         : UINT        = 0x00000400;
pub const WS_EX_RIGHT               : UINT        = 0x00001000;
pub const WS_EX_LEFT                : UINT        = 0x00000000;
pub const WS_EX_RTLREADING          : UINT        = 0x00002000;
pub const WS_EX_LTRREADING          : UINT        = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR       : UINT        = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR      : UINT        = 0x00000000;
pub const WS_EX_CONTROLPARENT       : UINT        = 0x00010000;
pub const WS_EX_STATICEDGE          : UINT        = 0x00020000;
pub const WS_EX_APPWINDOW           : UINT        = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW    : UINT        = (WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE);
pub const WS_EX_PALETTEWINDOW       : UINT        = (WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST);
pub const WS_EX_LAYERED             : UINT        = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT     : UINT        = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP : UINT        = 0x00200000;
pub const WS_EX_LAYOUTRTL           : UINT        = 0x00400000;
pub const WS_EX_COMPOSITED          : UINT        = 0x02000000;
pub const WS_EX_NOACTIVATE          : UINT        = 0x08000000;

pub struct WNDCLASSEXA {
	pub cbSize: UINT,
	pub style: UINT,
	pub lpfnWndProc: WNDPROC,
	pub cbClsExtra: INT,
	pub cbWndExtra: INT,
	pub hInstance: HINSTANCE,
	pub hIcon: HICON,
	pub hCursor: HCURSOR,
	pub hbrBackground: HBRUSH,
	pub lpszMenuName: LPCSTR,
	pub lpszClassName: LPCSTR,
	pub hIconSm: HICON
}

pub struct POINT {
    pub x: LONG,
    pub y: LONG
}

pub struct MSG {
    pub hWnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT
}

pub type LPMSG = *mut MSG;

