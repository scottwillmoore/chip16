use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use std::mem;

const ADDRESSABLE_MEMORY: usize = 65_536;
const ADDRESSABLE_VIDEO_MEMORY: usize = 76_800;

macro_rules! create_memory {
    ($name:ident, $size:expr) => {
        pub struct $name([u8; $size]);

        impl $name {
            pub fn new() -> $name {
                $name([0; $size])
            }

            pub fn reset(&mut self) {
                self.0 = [0; $size];
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
    };
}

create_memory!(Memory, ADDRESSABLE_MEMORY);
create_memory!(VideoMemory, ADDRESSABLE_VIDEO_MEMORY);
