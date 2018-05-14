// Consider refractoring Memory into a trait, so a various implementation can be made.
// E.g. MockMemory an implementation that does not actually execute any of the functions.
// E.g. FileMemory an implementation that saves all changes to file.

pub const ADDRESSABLE_MEMORY: usize = 65_536;

trait IndexBytesExt {
    fn read_u16(&self, index: usize) -> u16;
    fn write_u16(&mut self, index: usize, value: u16);
}

impl IndexBytesExt for [u8; ADDRESSABLE_MEMORY] {
    fn read_u16(&self, index: usize) -> u16 {
        ((self[index] as u16) << 8) & (self[index + 1] as u16)
    }

    fn write_u16(&mut self, index: usize, value: u16) {
        self[index] = (value >> 8 & 0xFF) as u8;
        self[index + 1] = (value & 0xFF) as u8;
    }
}

pub struct Memory {
    data: [u8; ADDRESSABLE_MEMORY],
}

impl Memory {
    fn new() -> Memory {
        Memory {
            data: [0; ADDRESSABLE_MEMORY],
        }
    }
}
