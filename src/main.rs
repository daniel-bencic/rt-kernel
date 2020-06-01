#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rt_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rt_kernel::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // loop {  }
    rt_kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rt_kernel::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    rt_kernel::init();

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    //loop {  }
    rt_kernel::hlt_loop();
}
