use core::{ops::Deref, ptr::NonNull};

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

unsafe impl Send for VgaBufferSync {}
unsafe impl Sync for VgaBufferSync {}

/// Safety of this struct assumes that there exists only one instance of it in the whole program.
pub struct VgaBufferRaw {
    vga: NonNull<u8>,
}

impl VgaBufferRaw {
    pub unsafe fn new() -> Self {
        VgaBufferRaw {
            vga: unsafe { NonNull::new_unchecked(0xB8000 as *mut u8) }
        }
    }

    pub unsafe fn write_character(&self, char: u8, color: u8) {
        unsafe {
            self.vga.offset(0).write_volatile(char);
            self.vga.offset(1).write_volatile(color);
        }
    }
}