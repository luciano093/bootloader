use crate::{asm_utils::{inb, outb}, vga::{VGA_HEIGHT, VGA_WIDTH}};

pub struct Cursor {
    col: u8,
    row: u8,
}

impl Cursor {
    pub(super) unsafe fn new(col: u8, row: u8) -> Self {
        unsafe { Self::enable(0, 15); }

        let mut cursor = Cursor {
            col,
            row,
        };

        unsafe { cursor.set_position(col, row); }

        cursor
    }

    /// `cursor_start` and `cursor_end` refer to the rows where the cursor starts and ends respectivelly. 
    /// The highest scanline is 0 and the lowest scanline is the maximum scanline (usually 15). 
    pub(super) unsafe fn enable(cursor_start: u8, cursor_end: u8) {
        unsafe {
            outb(0x3D4, 0x0A);
            outb(0x3D5, (inb(0x3D5) & 0xC0) | cursor_start);

            outb(0x3D4, 0x0B);
            outb(0x3D5, (inb(0x3D5) & 0xE0) | cursor_end);
        }
    }

    pub(super) unsafe fn _disable() {
        unsafe {
            outb(0x3D4, 0x0A);
	        outb(0x3D5, 0x20);
        }
    }

    pub(super) unsafe fn set_position(&mut self, col: u8, row: u8) {
        self.col = col;
        self.row = row;

        let pos = row as u16 * VGA_WIDTH as u16 + col as u16;

        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, (pos >> 8) as u8 & 0xFF);
        }
    }

    pub(super) fn advance(&mut self) {
        self.col += 1;

        if self.col >= VGA_WIDTH {
            self.col = 0;
            self.row += 1;

            if self.row >= VGA_HEIGHT {
                self.row = 0;
            }
        }

        unsafe { self.set_position(self.col, self.row); }
    }
}