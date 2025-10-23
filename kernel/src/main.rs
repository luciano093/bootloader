#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::vga::vga_buffer::VgaBuffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start() -> ! {
    loop {
        VgaBuffer::write_character('R', 0x04);
        VgaBuffer::write_character('U', 0x04);
        VgaBuffer::write_character('S', 0x04);
        VgaBuffer::write_character('T', 0x04);
    }
}