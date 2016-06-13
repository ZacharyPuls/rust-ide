extern crate libc;

use ::types::*;

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn GetModuleHandleA(
        lpModuleName: &'static str
    ) -> HMODULE;
}
