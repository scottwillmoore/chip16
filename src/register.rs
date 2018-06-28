const ADDRESSABLE_REGISTERS: usize = 16;

pub type Register = u16;

#[derive(Debug, PartialEq)]
pub struct RegisterFile([Register; ADDRESSABLE_REGISTERS]);

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile([0; ADDRESSABLE_REGISTERS])
    }

    pub fn reset(&mut self) {
        self.0 = [0; ADDRESSABLE_REGISTERS];
    }

    pub fn get<I: Into<usize>>(&self, index: I) -> &Register {
        self.0.get(index.into()).unwrap()
    }

    pub fn get_mut<I: Into<usize>>(&mut self, index: I) -> &mut Register {
        self.0.get_mut(index.into()).unwrap()
    }
}
