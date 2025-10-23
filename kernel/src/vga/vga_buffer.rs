use core::{ops::{Deref, DerefMut}, ptr::NonNull};

use spin::{Lazy, Mutex};


static VGA_RAW: Lazy<spin::Mutex<VgaBufferSync>> = Lazy::new(|| Mutex::new(VgaBufferSync(unsafe{ VgaBufferRaw::new() })));


pub struct VgaBuffer;

impl VgaBuffer {
    /// Writes a '?' character in place of characters that are not ascii.
    pub fn write_character(char: char, color: u8) {
        let byte = if char.is_ascii() { char as u8 } else { b'?' };

        unsafe {
            VGA_RAW.lock().write_character(byte, color);
        }
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
    cursor: u8, // represents the col the character is in. Max is 79
    line: u8, // represents the row the character is in. Max is 24
}

impl VgaBufferRaw {
    pub unsafe fn new() -> Self {
        VgaBufferRaw {
            vga: unsafe { NonNull::new_unchecked(0xB8000 as *mut u8) },
            cursor: 0,
            line: 0,
        }
    }

    pub unsafe fn write_character(&mut self, char: u8, color: u8) {
        let offset = (80  * self.line as isize + self.cursor as isize) * 2;

        unsafe {
            self.vga.offset(offset).write_volatile(char);
            self.vga.offset(offset  + 1).write_volatile(color);
        }

        self.advance_cursor();
    }

    fn advance_cursor(&mut self) {
        self.cursor += 1;

        // if cursor goes past its max, we advance the line and reset to 0
        if self.cursor >= 80 {
            self.advance_line();
            self.cursor = 0;
        }
    }

    fn advance_line(&mut self) {
        self.line += 1;

        // if line went past its max we wrap around to the top of the screen
        if self.line >= 25 {
            self.line = 0;
        }
    }
}