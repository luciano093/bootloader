use crate::{asm_utils::{inb, outb}, interrupt_handler, vga_print};

const SCANCODE_TO_ASCII: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', b'\x08', // 0x00-0x0E
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',   // 0x0F-0x1C
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',             // 0x1D-0x29
    0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',                   // 0x2A-0x35
    0, b'*', 0, b' ',                                                                        // 0x36-0x39
    // Fill rest with zeros
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
];


interrupt_handler!(keyboard_handler_asm, keyboard_handler);
#[unsafe(no_mangle)]
fn keyboard_handler() {
    // Reads scancode
    let scancode = unsafe { inb(0x60) };

    // ignores key releases
    // key releases cause an array out of bounds exception which make the kernel hang
    if scancode < 0x80 {
        let ascii = SCANCODE_TO_ASCII[scancode as usize];

        if ascii != 0 {
            vga_print!("{}", ascii as char);
        }
    }

    unsafe {
        outb(0x20, 0x20);
    }
}