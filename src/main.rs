// main program

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(wrinkly_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use wrinkly_os::println;
use wrinkly_os::print_logo;


#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello world!");
    wrinkly_os::init();

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    
    //print_logo();

    loop {
        use wrinkly_os::print;
        for _ in 0..10000{}
        print!("-");
    }
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
