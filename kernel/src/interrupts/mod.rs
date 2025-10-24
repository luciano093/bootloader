use crate::asm_utils::outb;

pub mod hardware_irqs;
pub mod idt;

#[macro_export]
macro_rules! interrupt_handler {
    ($name:ident, $rust_fn:ident) => {
        // Generate assembly wrapper
        core::arch::global_asm!(
            concat!(
                ".global ", stringify!($name), "\n",
                stringify!($name), ":\n",
                "    pushad\n",
                "    call ", stringify!($rust_fn), "\n",
                "    popad\n",
                "    iretd\n"
            )
        );
        
        unsafe extern "C" {
            pub unsafe fn $name();
        }
    };
}

interrupt_handler!(default_handler_asm, default_handler_rust);
#[unsafe(no_mangle)]
fn default_handler_rust() { }

pub unsafe fn remap_pic() {
    unsafe {
        // Start initialization
        outb(0x20, 0x11);
        outb(0xA0, 0x11);
        
        // Set vector offsets
        outb(0x21, 32);    // (IRQ 0-7 → interrupts 32-39)
        outb(0xA1, 40);    // (IRQ 8-15 → interrupts 40-47)
        
        // Tell PICs about each other
        outb(0x21, 0x04);
        outb(0xA1, 0x02);  
        
        // Set 8086 mode mode
        outb(0x21, 0x01); 
        outb(0xA1, 0x01);
        
        // Restore initial values
        outb(0x21, 0xFF);
        outb(0xA1, 0xFF);
    }
}