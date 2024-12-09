use x86_64::instructions::segmentation;
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::PrivilegeLevel;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ExceptionType {
    DivisionError = 0x0,
    Debug = 0x1,
    NonMaskableInterrupt = 0x2,
    Breakpoint = 0x3,
    Overflow = 0x4,
    BoundRangeExceeded = 0x5,
    InvalidOpcode = 0x6,
    DeviceNotAvailable = 0x7,
    DoubleFault = 0x8,
    InvalidTSS = 0xa,
    SegmentNotPresent = 0xb,
    StackSegmentFault = 0xc,
    GeneralProtectionFault = 0xd,
    PageFault = 0xe,
    X87FloatingPointException = 0x10,
    AlignmentCheck = 0x11,
    MachineCheck = 0x12,
    SIMDFloatingPointException = 0x13,
    VirtualizationException = 0x14,
    ControlProtectionException = 0x15,
    HypervisorInjectionException = 0x1c,
    VMMCommunicationException = 0x1d,
    SecurityException = 0x1e,
}

#[repr(C, packed)]
pub struct Idt([Entry; 16]);

impl Idt {
    pub fn new() -> Idt {
        Idt([Entry::missing(); 16])
    }

    pub fn set_handler(&mut self, entry: ExceptionType, handler: HandlerFunc) -> &mut EntryOptions {
        self.0[entry as usize] = Entry::new(segmentation::CS::get_reg(), handler);
        let options_ptr = core::ptr::addr_of_mut!(self.0[entry as usize].options);
        unsafe {
            &mut *options_ptr 
        }
    }
    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lidt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: x86_64::VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16
        };
        
        unsafe { lidt(&ptr) };
    }
}


#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32
}

use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut options = 0;
        options.set_bits(9..12, 0b111);
        EntryOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }
    pub fn set_privilage_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_bits(13..15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index);
        self
    }
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16 ) as u16,
            pointer_high: (pointer >> 32 ) as u32,
            options: EntryOptions::new(),
            reserved: 0
        }
    }
    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
                pointer_low: 0,
                pointer_middle: 0,
                pointer_high: 0,
                options: EntryOptions::minimal(),
                reserved: 0,
        }
    }
}

pub type HandlerFunc = extern "C" fn() -> !;
pub type HandlerFuncWithErr = extern "C" fn(InterruptStackFrame, u64) -> !;

