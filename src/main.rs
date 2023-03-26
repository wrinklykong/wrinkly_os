// main program

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(wrinkly_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use wrinkly_os::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    wrinkly_os::init();
    
    //fn stack_overflow() {
    //    stack_overflow();
    //}

    //stack_overflow();
    
    #[cfg(test)]
    test_main();

    println!("it did not crash!");

    loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wrinkly_os::test_panic_handler(info)
}
