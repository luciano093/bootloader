#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use kernel::{asm_utils::{inb, outb}, interrupts::{idt::IdtDescriptor, remap_pic}, vga_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start() -> ! {
    vga_println!("aaa");
    
    let descriptor = IdtDescriptor::new();
    
    unsafe { descriptor.load(); }
    
    unsafe { remap_pic() };
    
    unsafe {
        let mask = inb(0x21);
        outb(0x21, mask & !0x02);  // Clear bit 1
    }

    unsafe { asm!("sti"); }
    
    loop { }
}