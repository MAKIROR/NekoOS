#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(neko_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use neko_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    neko_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    neko_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    neko_os::hlt_loop();
}