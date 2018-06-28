use std::mem;

pub trait Load<I, V> {
    fn load(&self, index: I) -> V {
        let mut value;
        unsafe {
            value = mem::uninitialized();
        }
        self.load_into(index, &mut value);
        value
    }

    fn load_into(&self, index: I, value: &mut V);
}

pub trait Store<I, V> {
    fn store(&mut self, index: I, value: V);
}

pub const ADDRESSABLE_MEMORY: usize = 65_536;

pub struct Memory([u8; ADDRESSABLE_MEMORY]);

impl Memory {
    pub fn new() -> Memory {
        Memory([0; ADDRESSABLE_MEMORY])
    }
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new()
    }
}

impl<I: Into<usize>> Load<I, u8> for Memory {
    fn load_into(&self, index: I, value: &mut u8) {
        *value = self.0[index.into()]
    }
}

impl<I: Into<usize>> Store<I, u8> for Memory {
    fn store(&mut self, index: I, value: u8) {
        self.0[index.into()] = value;
    }
}

impl<I: Into<usize>> Load<I, u16> for Memory {
    fn load_into(&self, index: I, value: &mut u16) {
        let index = index.into();
        *value = (self.0[index] as u16) & ((self.0[index + 1] as u16) << 8)
    }
}

impl<I: Into<usize>> Store<I, u16> for Memory {
    fn store(&mut self, index: I, value: u16) {
        let index = index.into();
        self.0[index] = (value & 0xFF) as u8;
        self.0[index + 1] = (value & 0xFF00 >> 8) as u8;
    }
}

impl<I: Into<usize>> Load<I, u32> for Memory {
    fn load_into(&self, index: I, value: &mut u32) {
        let index = index.into();
        *value = (self.0[index] as u32) & ((self.0[index + 1] as u32) << 8)
            & ((self.0[index + 2] as u32) << 16)
            & ((self.0[index + 3] as u32) << 24)
    }
}

impl<I: Into<usize>> Store<I, u32> for Memory {
    fn store(&mut self, index: I, value: u32) {
        let index = index.into();
        self.0[index] = (value & 0xFF) as u8;
        self.0[index + 1] = (value & 0xFF00 >> 8) as u8;
        self.0[index + 2] = (value & 0xFF0000 >> 16) as u8;
        self.0[index + 3] = (value & 0xFF000000 >> 24) as u8;
    }
}

// TODO: Check if this code is okay..?
use std::io::Write;
impl<'a, I: Into<usize>> Store<I, &'a Vec<u8>> for Memory {
    fn store(&mut self, index: I, value: &'a Vec<u8>) {
        let mut index = index.into();
        for byte in value {
            println!("{}", byte);
            self.0[index] = *byte;
            println!("{}", self.0[index]);
            index += 1;
        }
    }
}

pub const ADDRESSABLE_VIDEO_MEMORY: usize = 320 * 240;

pub struct VideoMemory([u8; ADDRESSABLE_VIDEO_MEMORY]);

impl VideoMemory {
    pub fn new() -> VideoMemory {
        VideoMemory([0; ADDRESSABLE_VIDEO_MEMORY])
    }
}

impl Default for VideoMemory {
    fn default() -> VideoMemory {
        VideoMemory::new()
    }
}

impl<I: Into<usize>> Load<I, u8> for VideoMemory {
    fn load_into(&self, index: I, value: &mut u8) {
        *value = self.0[index.into()]
    }
}

impl<I: Into<usize>> Store<I, u8> for VideoMemory {
    fn store(&mut self, index: I, value: u8) {
        self.0[index.into()] = value;
    }
}

impl<I: Into<usize>> Load<I, u16> for VideoMemory {
    fn load_into(&self, index: I, value: &mut u16) {
        let index = index.into();
        *value = (self.0[index] as u16) & ((self.0[index + 1] as u16) << 8)
    }
}

impl<I: Into<usize>> Store<I, u16> for VideoMemory {
    fn store(&mut self, index: I, value: u16) {
        let index = index.into();
        self.0[index] = (value & 0xFF) as u8;
        self.0[index + 1] = (value & 0xFF00 >> 8) as u8;
    }
}

impl<I: Into<usize>> Load<I, u32> for VideoMemory {
    fn load_into(&self, index: I, value: &mut u32) {
        let index = index.into();
        *value = (self.0[index] as u32) & ((self.0[index + 1] as u32) << 8)
            & ((self.0[index + 2] as u32) << 16)
            & ((self.0[index + 3] as u32) << 24)
    }
}

impl<I: Into<usize>> Store<I, u32> for VideoMemory {
    fn store(&mut self, index: I, value: u32) {
        let index = index.into();
        self.0[index] = (value & 0xFF) as u8;
        self.0[index + 1] = (value & 0xFF00 >> 8) as u8;
        self.0[index + 2] = (value & 0xFF0000 >> 16) as u8;
        self.0[index + 3] = (value & 0xFF000000 >> 24) as u8;
    }
}
