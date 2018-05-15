// Consider refractoring Memory into a trait, so a various implementation can be made.
// E.g. MockMemory an implementation that does not actually execute any of the functions.
// E.g. FileMemory an implementation that saves all changes to file.
use std::default::Default;
use std::mem;

pub const ADDRESSABLE_MEMORY: usize = 65_536;

pub trait Read<I, V> {
    fn read(&self, index: I) -> V {
        let mut value;
        unsafe {
            value = mem::uninitialized();
        }
        self.read_into(index, &mut value);
        value
    }

    fn read_into(&self, index: I, value: &mut V);
}

pub trait Write<I, V> {
    fn write(&mut self, index: I, value: V);
}

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

impl<I: Into<usize>> Read<I, u8> for Memory {
    fn read_into(&self, index: I, value: &mut u8) {
        *value = self.0[index.into()]
    }
}

impl<I: Into<usize>> Write<I, u8> for Memory {
    fn write(&mut self, index: I, value: u8) {
        self.0[index.into()] = value;
    }
}

impl<I: Into<usize>> Read<I, u16> for Memory {
    fn read_into(&self, index: I, value: &mut u16) {
        let index = index.into();
        *value = (self.0[index] as u16) & ((self.0[index + 1] as u16) << 8)
    }
}

impl<I: Into<usize>> Write<I, u16> for Memory {
    fn write(&mut self, index: I, value: u16) {
        let index = index.into();
        self.0[index] = (value & 0xFF) as u8;
        self.0[index + 1] = (value & 0xFF00 >> 8) as u8;
    }
}

impl<I: Into<usize>> Read<I, u32> for Memory {
    fn read_into(&self, index: I, value: &mut u32) {
        let index = index.into();
        *value = (self.0[index] as u32) & ((self.0[index + 1] as u32) << 8)
            & ((self.0[index + 2] as u32) << 16)
            & ((self.0[index + 3] as u32) << 24)
    }
}

impl<I: Into<usize>> Write<I, u32> for Memory {
    fn write(&mut self, index: I, value: u32) {
        let index = index.into();
        self.0[index] = (value & 0xFF) as u8;
        self.0[index + 1] = (value & 0xFF00 >> 8) as u8;
        self.0[index + 2] = (value & 0xFF0000 >> 16) as u8;
        self.0[index + 3] = (value & 0xFF000000 >> 24) as u8;
    }
}
