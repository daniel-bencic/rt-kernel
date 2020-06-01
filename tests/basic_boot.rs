#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rt_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rt_kernel::{println, serial_print, serial_println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rt_kernel::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}
