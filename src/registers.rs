use std::ops::{Index, IndexMut};

pub const ADDRESSABLE_REGISTERS: usize = 16;

#[derive(Default)]
pub struct Registers([u16; ADDRESSABLE_REGISTERS]);

impl Registers {
    pub fn new() -> Registers {
        Registers([0; ADDRESSABLE_REGISTERS])
    }
}

impl<T: Into<usize>> Index<T> for Registers {
    type Output = u16;

    fn index(&self, index: T) -> &u16 {
        &self.0[index.into()]
    }
}

impl<T: Into<usize>> IndexMut<T> for Registers {
    fn index_mut(&mut self, index: T) -> &mut u16 {
        &mut self.0[index.into()]
    }
}
