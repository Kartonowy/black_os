use idt::HandlerFunc;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;

use crate::println;

mod idt;

pub fn init() {
    IDT.load();
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        unsafe {
            idt.Breakpoint.set_handler(breakpoint_handler());
        }

        idt
    };
}

extern "C" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

extern "C" fn breakpoint_handler() -> ! {
    println!("EXCEPTION: BREAKPOINT");
    loop {}
}

extern "C" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", _error_code)
}
