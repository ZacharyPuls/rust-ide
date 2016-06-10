extern crate winapi;
extern crate gui;

use winapi::types::HBRUSH;

use gui::window::create_window;

pub const VERSION: &'static str = "0.1.0";

fn disp_callback() {
    println!("RustIDE [ display callback ]");
}

#[link_args = "-Wl,--subsystem,windows"]
fn main() {
    println!("RustIDE Version {}", VERSION);
    unsafe {
        create_window("RustIDE", 300, 300, 640, 480, 5 as HBRUSH);
    }
}
