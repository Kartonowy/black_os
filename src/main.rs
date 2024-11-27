#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

use core::{arch::asm, panic::PanicInfo};

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    println!("Hello world");

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assert() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[OK]");
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}


pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        unsafe {
            asm!("out dx, eax", in("dx") 0xf4, in("eax") exit_code as u32, options(nomem, nostack, preserves_flags));
        }
    }
}
