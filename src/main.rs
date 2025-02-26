#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_minimal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_minimal_os::println;




#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    println!("Hello World!!!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_minimal_os::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    use core::panic;

    assert_eq!(1, 1);
}