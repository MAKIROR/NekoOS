#![feature(panic_handler)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate bootloader;

#[path = "mods/vga_buffer.rs"]
mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}