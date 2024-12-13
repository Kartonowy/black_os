#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(black_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use black_os::divide_by_zero;
use black_os::println;
use core::panic::PanicInfo;
use x86_64::instructions::interrupts::int3;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    black_os::init();

    unsafe { *(0xdeadbeef as *mut u8) = 42 }

    #[cfg(test)]
    test_main();

    println!("id didnt crash?!?!");
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
