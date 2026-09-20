mod gui_v2;
mod core;
mod device_manager;
mod scheduler;
mod ffi;

use core::SignitifyCore;

fn main() {
    let core = SignitifyCore::new();
    gui_v2::run(core);
}