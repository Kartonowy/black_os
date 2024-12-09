#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(black_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use black_os::{exit_qemu, interrupts, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    interrupts::init();

    let _ = 42 / 0;

    serial_println!("[divide by zero not caught]");
    exit_qemu(black_os::QemuExitCode::Failed);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(black_os::QemuExitCode::Success);
    loop {}
}
