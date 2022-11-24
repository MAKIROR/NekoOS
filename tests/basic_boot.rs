#![test_runner(neko_os::test_runner)]

use neko_os::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    neko_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}