use core::arch::asm;

use spin::Lazy;

use crate::{interrupts::{default_handler_asm, hardware_irqs::keyboard_handler_asm}};

static IDT: Lazy<Idt> = Lazy::new(|| {
    let mut idt = Idt::new();

    for i in 0..256 {
        idt.set_handler(i, default_handler_asm as u32, 0x8E);
    }

    idt.set_handler(33, keyboard_handler_asm as u32, 0x8E);

    idt
});



struct Idt {
    entries: [IdtEntry; 256]
}

impl Idt {
    fn new() -> Self {
        Self { entries: [IdtEntry::empty(); 256] }
    }

    fn set_handler(&mut self, index: usize, address: u32, gate_type: u8) {
        let entry = IdtEntry {
            offset_low: (address & 0xFFFF) as u16,
            selector: 0x08,
            zero: 0,
            gate_type,
            offset_high: (address >> 16) as u16,
        };

        self.entries[index] = entry;
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct IdtEntry {
    offset_low: u16,          // Low 16 bits of address to call
    selector: u16,             // kernel code segment from GDT
    zero: u8,                 // Always zero (reserved)
    gate_type: u8,            // Gate type to use
    offset_high: u16,         // High 16 bits of address to call
}

impl IdtEntry {
    pub const fn empty() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            zero: 0,
            gate_type: 0,
            offset_high: 0,
        }
    }
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    pub limit: u16, // size of IDT - 1 (256 * 8 - 1)
    pub base: u32, // address of IDT array
}

impl IdtDescriptor {
    pub fn new() -> IdtDescriptor {
        // Force Lazy to initialize idt
        let idt_ref = &*IDT;

        Self { limit: (256 * 8 - 1) as u16, base: core::ptr::addr_of!(idt_ref.entries) as u32 }
    }

    pub unsafe fn load(&self) {
            let local_desc = IdtDescriptor {
        limit: self.limit,
        base: self.base,
    };

        unsafe {
            asm!(
                "lidt [eax]",
                in("eax") &local_desc as *const _,
                options(nostack, preserves_flags)
            );
        }
    }
}