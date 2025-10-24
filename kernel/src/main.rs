#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::{vga_print, vga_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start() -> ! {
    vga_println!("My string: {}", 1);
    vga_println!("My string: {}", 1);
    vga_print!("My string: {}", 1);
    vga_print!(" + 2 = 3");
    vga_println!();
    vga_println!();
    vga_print!(":D");


    
    loop { }
}