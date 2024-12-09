#![allow(dead_code)]
use core::marker::PhantomData;


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

#[allow(non_snake_case)]
#[repr(C)]
#[repr(align(16))]
pub struct Idt {
    pub DivisionError: Entry<HandlerFunc>,
    pub Debug: Entry<HandlerFunc>,
    pub NonMaskableInterrupt: Entry<HandlerFunc>,
    pub Breakpoint: Entry<HandlerFunc>,
    pub Overflow: Entry<HandlerFunc>,
    pub BoundRangeExceeded: Entry<HandlerFunc>,
    pub InvalidOpcode: Entry<HandlerFunc>,
    pub DeviceNotAvailable: Entry<HandlerFunc>,
    pub DoubleFault: Entry<HandlerFuncWithErr>,
    pub InvalidTSS: Entry<HandlerFuncWithErr>,
    pub SegmentNotPresent: Entry<HandlerFuncWithErr>,
    pub StackSegmentFault: Entry<HandlerFuncWithErr>,
    pub GeneralProtectionFault: Entry<HandlerFuncWithErr>,
    pub PageFault: Entry<HandlerFuncWithErr>,
    pub X87FloatingPointException: Entry<HandlerFunc>,
    pub AlignmentCheck: Entry<HandlerFuncWithErr>,
    pub MachineCheck: Entry<HandlerFunc>,
    pub SIMDFloatingPointException: Entry<HandlerFunc>,
    pub VirtualizationException: Entry<HandlerFunc>,
    pub ControlProtectionException: Entry<HandlerFuncWithErr>,
    pub HypervisorInjectionException: Entry<HandlerFunc>,
    pub VMMCommunicationException: Entry<HandlerFuncWithErr>,
    pub SecurityException: Entry<HandlerFuncWithErr>,
}

impl Idt {
    pub fn new() -> Idt {
        Idt {
            DivisionError: Entry::missing(),
            Debug: Entry::missing(),
            NonMaskableInterrupt: Entry::missing(),
            Breakpoint: Entry::missing(),
            Overflow: Entry::missing(),
            BoundRangeExceeded: Entry::missing(),
            InvalidOpcode: Entry::missing(),
            DeviceNotAvailable: Entry::missing(),
            DoubleFault: Entry::missing(),
            InvalidTSS: Entry::missing(),
            SegmentNotPresent: Entry::missing(),
            StackSegmentFault: Entry::missing(),
            GeneralProtectionFault: Entry::missing(),
            PageFault: Entry::missing(),
            X87FloatingPointException: Entry::missing(),
            AlignmentCheck: Entry::missing(),
            MachineCheck: Entry::missing(),
            SIMDFloatingPointException: Entry::missing(),
            VirtualizationException: Entry::missing(),
            ControlProtectionException: Entry::missing(),
            HypervisorInjectionException: Entry::missing(),
            VMMCommunicationException: Entry::missing(),
            SecurityException: Entry::missing(),
        }
    }

    pub fn load(&'static self) {
        use core::mem::size_of;
        use x86_64::instructions::tables::{lidt, DescriptorTablePointer};

        let ptr = DescriptorTablePointer {
            base: x86_64::VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<F>
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

impl<F> Entry<F> {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
            phantom: PhantomData
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
            phantom: PhantomData
        }
    }

    pub unsafe fn set_handler(&mut self, handler: F)  -> &mut EntryOptions 
        where F: HandlerType {
        let address = handler.to_virt_addr().0;

        self.pointer_low = address as u16;
        self.pointer_middle = (address >> 16) as u16;
        self.pointer_high = (address >> 32) as u32;

        self.options = EntryOptions::minimal();
        self.gdt_selector = segmentation::CS::get_reg();

        &mut self.options
    }
}

pub unsafe trait HandlerType {
    fn to_virt_addr(self) -> VirtAddr;
}

#[repr(transparent)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn new(addr: u64) -> VirtAddr {
        VirtAddr(addr)
    }
}


pub type HandlerFunc = extern "C" fn(InterruptStackFrame) -> ! ;
unsafe impl HandlerType for HandlerFunc {
    fn to_virt_addr(self) -> VirtAddr {
        VirtAddr(self as u64)
    }
}
pub type HandlerFuncWithErr = extern "C" fn(InterruptStackFrame, u64) -> !;
unsafe impl HandlerType for HandlerFuncWithErr {
    fn to_virt_addr(self) -> VirtAddr {
        VirtAddr(self as u64)
    }
}
