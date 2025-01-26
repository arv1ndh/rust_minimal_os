#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello World!!").unwrap();
    write!(vga_buffer::WRITER.lock(), "Test for other data like numbers: {}, {}", 5, 2.0/3.0);
    write!(vga_buffer::WRITER.lock(), "This is a test for newline to make sure the rows are pushed up");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}