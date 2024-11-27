#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HI: &[u8] = b"Hiii!!";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u8;

    for (i, &byte) in HI.iter().enumerate() {
        unsafe {
            *vga.offset(i as isize * 2) = byte;
            *vga.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
