#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello from main").unwrap();
    // write!(vga_buffer::WRITER.lock(), "impossivel funcionar {},{}", 1, 2).unwrap();

    loop{ vga_buffer::WRITER.lock().write_str("Hello from main").unwrap();}
}
 
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
