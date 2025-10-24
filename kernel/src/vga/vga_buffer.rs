use core::{fmt::{self, Write}, ops::{Deref, DerefMut}, ptr::NonNull};

use spin::{Lazy, Mutex};

use crate::vga::{cursor::Cursor, VGA_HEIGHT, VGA_WIDTH};

static VGA_RAW: Lazy<spin::Mutex<VgaBufferSync>> = Lazy::new(|| Mutex::new(VgaBufferSync(unsafe{ VgaBufferRaw::new() })));

static DEFAULT_COLOR: u8 = 0x0F;

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => {{
        use $crate::vga::vga_buffer::VgaBuffer;
        VgaBuffer::print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! vga_println {
    () => {{
        use $crate::vga::vga_buffer::VgaBuffer;
        VgaBuffer::write_character('\n');
    }};

    ($($arg:tt)*) => {{
        use $crate::vga::vga_buffer::VgaBuffer;

        VgaBuffer::print(format_args!($($arg)*));
        VgaBuffer::write_character('\n');
    }};
}



pub struct VgaBuffer;

impl VgaBuffer {
    /// Writes a '?' character in place of characters that are not ascii.
    pub fn write_character_with_color(char: char, color: u8) {
        unsafe {
            VGA_RAW.lock().write_character_with_color(char, color);
        }
    }

    pub fn write_character(char: char) {
        unsafe {
            VGA_RAW.lock().write_character(char);
        }
    }

    pub fn print(args: fmt::Arguments) {
        let _ = VGA_RAW.lock().write_fmt(args);
    }

    pub fn println(args: fmt::Arguments) {
        let _ = VGA_RAW.lock().write_fmt(args);
    }

    pub fn clear() {
        let _ = VGA_RAW.lock().clear();
    }
}

/// Private thin wrapper to be able to put VgaBuggerRaw in a singleton mutex to use for the safe abstraction
#[repr(transparent)]
struct VgaBufferSync(VgaBufferRaw);

impl Deref for VgaBufferSync {
    type Target = VgaBufferRaw;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VgaBufferSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl Send for VgaBufferSync {}
unsafe impl Sync for VgaBufferSync {}

/// Safety of this struct assumes that there exists only one instance of it in the whole program.
pub struct VgaBufferRaw {
    vga: NonNull<u8>,
    current_col: u8, // represents the col the character is in. Max is 79
    current_row: u8, // represents the row the character is in. Max is 24
    cursor: Cursor,
}

impl VgaBufferRaw {
    pub unsafe fn new() -> Self {
        let cursor = unsafe { Cursor::new(0, 0) };

        VgaBufferRaw {
            vga: unsafe { NonNull::new_unchecked(0xB8000 as *mut u8) },
            current_col: 0,
            current_row: 0,
            cursor,
        }
    }

    pub unsafe fn write_str_with_color(&mut self, str: &str, color: u8) {
        for char in str.chars() {
            unsafe { self.write_character_with_color(char, color); }
        }
    }

    pub unsafe fn write_character(&mut self, char: char) {
        unsafe { self.write_character_with_color(char, DEFAULT_COLOR); }
    }

    pub unsafe fn write_character_with_color(&mut self, char: char, color: u8) {
        if char == '\n' {
            self.advance_current_row();
            self.current_col = 0;
            unsafe { self.cursor.set_position(self.current_col, self.current_row); }
            return;
        }

        let byte = if char.is_ascii() { char as u8 } else { b'?' };

        let offset = (VGA_WIDTH as isize  * self.current_row as isize + self.current_col as isize) * 2;

        unsafe {
            self.vga.offset(offset).write_volatile(byte);
            self.vga.offset(offset  + 1).write_volatile(color);
        }

        self.advance_current_col();
        self.cursor.advance();
    }

    fn advance_current_col(&mut self) {
        self.current_col += 1;

        // if current_col goes past its max, we advance the current_row and reset to 0
        if self.current_col >= VGA_WIDTH {
            self.advance_current_row();
            self.current_col = 0;
        }
    }

    fn advance_current_row(&mut self) {
        self.current_row += 1;

        // if current_row went past its max we wrap around to the top of the screen
        if self.current_row >= VGA_HEIGHT {
            self.current_row = 0;
        }
    }

    pub fn clear(&mut self) {
        for offset in 0..=(VGA_WIDTH as isize * VGA_HEIGHT as isize) {
            let offset = offset * 2;
            unsafe {
                self.vga.offset(offset as isize).write_volatile(b' ');      // character
                self.vga.offset(offset as isize + 1).write_volatile(0x0F); // color
            }
        }

        self.current_col = 0;
        self.current_row = 0;
        unsafe { self.cursor.set_position(0, 0) };
    }
}

impl Write for VgaBufferRaw {
    fn write_str(&mut self, str: &str) -> core::fmt::Result {
        unsafe { self.write_str_with_color(str, DEFAULT_COLOR); }

        Ok(())
    }
}