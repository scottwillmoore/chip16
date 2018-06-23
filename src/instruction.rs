use failure::Error;

use self::Condition::*;
use self::Operation::*;

#[derive(Debug, PartialEq)]
pub enum Operation {
    NOP,
    JMPI,
    LDIR,
}

impl Operation {
    pub fn new(data: u8) -> Result<Operation, Error> {
        match data {
            0x00 => Ok(NOP),
            0x01 => Ok(JMPI),
            0x02 => Ok(LDIR),
            _ => Err(format_err!(
                "failed to decode 0x{:02x} into an instruction",
                data
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Z,
    NZ,
    N,
    NN,
    P,
    O,
    NO,
    A,
    AE,
    B,
    BE,
    G,
    GE,
    L,
    LE,
}

impl Condition {
    pub fn new(data: u8) -> Result<Condition, Error> {
        match data {
            0x0 => Ok(Z),
            0x1 => Ok(NZ),
            0x2 => Ok(N),
            0x3 => Ok(NN),
            0x4 => Ok(P),
            0x5 => Ok(O),
            0x6 => Ok(NO),
            0x7 => Ok(A),
            0x8 => Ok(AE),
            0x9 => Ok(B),
            0xA => Ok(BE),
            0xB => Ok(G),
            0xC => Ok(GE),
            0xD => Ok(L),
            0xE => Ok(LE),
            _ => Err(format_err!(
                "failed to decode 0x{:02x} into a condition",
                data
            )),
        }
    }
}

macro_rules! extract_bits {
    ($f:ident, 4, $i:expr) => {
        pub fn $f(&self) -> u8 {
            let mask = 0xF;
            let shift = $i * 4;
            ((self.0 & (mask << shift)) >> shift) as u8
        }
    };
    ($f:ident, 8, $i:expr) => {
        pub fn $f(&self) -> u8 {
            let mask = 0xFF;
            let shift = $i * 8;
            ((self.0 & (mask << shift)) >> shift) as u8
        }
    };
    ($f:ident, 16, $i:expr) => {
        pub fn $f(&self) -> u16 {
            let mask = 0xFFFF;
            let shift = $i * 16;
            ((self.0 & (mask << shift)) >> shift) as u16
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct Instruction(u32);
// pub struct Instruction(Operation, u8, u8, u8);

// pub struct Arguments(u8, u8, u8);
// pub struct Instruction(Operation, Arguments);

impl Instruction {
    pub fn new(data: u32) -> Instruction {
        Instruction(data)
    }

    pub fn operation(&self) -> Result<Operation, Error> {
        Operation::new(self.ii())
    }

    pub fn condition(&self) -> Result<Condition, Error> {
        Condition::new(self.x())
    }

    extract_bits!(hhll, 16, 0);
    extract_bits!(hh, 8, 1);
    extract_bits!(ll, 8, 0);
    extract_bits!(ii, 8, 3);
    extract_bits!(n, 4, 3);
    extract_bits!(x, 4, 4);
    extract_bits!(y, 4, 5);
    extract_bits!(z, 4, 2);
}
