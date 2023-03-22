// main program

#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;         // includes vga_buffer.rs

static HELLO: &[u8] = b"Hello this is my OS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
