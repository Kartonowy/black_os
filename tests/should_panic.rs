#![no_std]
#![no_main]

use core::panic::PanicInfo;

use imsorry::{exit_qemu, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(imsorry::QemuExitCode::Failed);

    loop {}
}

fn should_fail() {
    serial_println!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]"); 
    exit_qemu(imsorry::QemuExitCode::Success);
    loop {}
}

