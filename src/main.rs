#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(imsorry::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use imsorry::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for _ in 0..100 {
        println!("Hello world");
    }

    #[cfg(test)]
    test_main();

    loop {}
}




#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    imsorry::test_panic_handler(_info);
}
