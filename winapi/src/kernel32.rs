extern crate libc;

use ::types::*;

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn GetModuleHandleW(
        lpModuleName: String
    ) -> HMODULE;
}
