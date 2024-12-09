#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(black_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use black_os::divide_by_zero;
use black_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    black_os::init();


    // divide_by_zero();

    x86_64::instructions::interrupts::int3();

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
    black_os::test_panic_handler(_info);
}
