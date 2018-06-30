use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use std::mem;

const ADDRESSABLE_MEMORY: usize = 65_536;

pub struct Memory([u8; ADDRESSABLE_MEMORY]);

impl Memory {
    pub fn new() -> Memory {
        Memory([0; ADDRESSABLE_MEMORY])
    }

    pub fn clear(&mut self) {
        self.0 = [0; ADDRESSABLE_MEMORY];
    }

    // read_sprite
    // write_sprite

    pub fn read_u8<I: Into<usize>>(&self, index: I) -> u8 {
        *self.0.get(index.into()).unwrap()
    }

    pub fn read_u16<I: Into<usize>>(&self, index: I) -> u16 {
        let index = index.into();
        let width = mem::size_of::<u16>();
        let mut buf = self.0.get(index..index + width).unwrap();
        buf.read_u16::<LittleEndian>().unwrap()
    }

    pub fn read_u32<I: Into<usize>>(&self, index: I) -> u32 {
        let index = index.into();
        let width = mem::size_of::<u32>();
        let mut buf = self.0.get(index..index + width).unwrap();
        buf.read_u32::<LittleEndian>().unwrap()
    }

    pub fn write_u8<I: Into<usize>>(&mut self, index: I, value: u8) {
        *self.0.get_mut(index.into()).unwrap() = value;
    }

    pub fn write_u16<I: Into<usize>>(&mut self, index: I, value: u16) {
        let index = index.into();
        let width = mem::size_of::<u16>();
        let mut buf = self.0.get_mut(index..index + width).unwrap();
        buf.write_u16::<LittleEndian>(value).unwrap()
    }

    pub fn write_u32<I: Into<usize>>(&mut self, index: I, value: u32) {
        let index = index.into();
        let width = mem::size_of::<u32>();
        let mut buf = self.0.get_mut(index..index + width).unwrap();
        buf.write_u32::<LittleEndian>(value).unwrap()
    }
}
