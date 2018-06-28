#[derive(Debug, PartialEq)]
pub enum Operation {
    NOP,
    CLS,
    VBLNK,
    BGC,
    SPR,
    DRWI,
    DRWR,
    RND,
    FLIP,
    SND0,
    SND1,
    SND2,
    SND3,
    SNP,
    SNG,
    JMPI,
    JMC,
    JX,
    JME,
    CALLI,
    RET,
    JMPR,
    CX,
    CALLR,
    LDIR,
    LDIS,
    LDMI,
    LDMR,
    MOV,
    STMI,
    STMR,
    ADDI,
    ADDR2,
    ADDR3,
    SUBI,
    SUBR2,
    SUBR3,
    CMPI,
    CMPR,
    ANDI,
    ANDR2,
    ANDR3,
    TSTI,
    TSTR,
    ORI,
    ORR2,
    ORR3,
    XORI,
    XORR2,
    XORR3,
    MULI,
    MULR2,
    MULR3,
    DIVI,
    DIVR2,
    DIVR3,
    MODI,
    MODR2,
    MODR3,
    REMI,
    REMR2,
    REMR3,
    SHLN,
    SHRN,
    SARN,
    SHLR,
    SHRR,
    SARR,
    PUSH,
    POP,
    PUSHALL,
    POPALL,
    PUSHF,
    POPF,
    PALI,
    PALR,
    NOTI,
    NOTR1,
    NOTR2,
    NEGI,
    NEGR1,
    NEGR2,
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

macro_rules! extract_argument {
    ($name:ident, $output:ty, $width:expr, $index:expr) => {
        pub fn $name(&self) -> $output {
            let mask = (1 << $width) - 1;
            let shift = $index * $width;
            let aligned_mask = mask << shift;
            ((self.0 & aligned_mask) >> shift) as $output
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct Instruction(pub u32);

impl Instruction {
    pub fn new(data: u32) -> Instruction {
        Instruction(data)
    }

    pub fn decode_operation(&self) -> Option<Operation> {
        match self.ii() {
            0x00 => Some(Operation::NOP),
            0x01 => Some(Operation::CLS),
            0x02 => Some(Operation::VBLNK),
            0x03 => Some(Operation::BGC),
            0x04 => Some(Operation::SPR),
            0x05 => Some(Operation::DRWI),
            0x06 => Some(Operation::DRWR),
            0x07 => Some(Operation::RND),
            0x08 => Some(Operation::FLIP),
            0x09 => Some(Operation::SND0),
            0x0A => Some(Operation::SND1),
            0x0B => Some(Operation::SND2),
            0x0C => Some(Operation::SND3),
            0x0D => Some(Operation::SNP),
            0x0E => Some(Operation::SNG),
            0x10 => Some(Operation::JMPI),
            0x11 => Some(Operation::JMC),
            0x12 => Some(Operation::JX),
            0x13 => Some(Operation::JME),
            0x14 => Some(Operation::CALLI),
            0x15 => Some(Operation::RET),
            0x16 => Some(Operation::JMPR),
            0x17 => Some(Operation::CX),
            0x18 => Some(Operation::CALLR),
            0x20 => Some(Operation::LDIR),
            0x21 => Some(Operation::LDIS),
            0x22 => Some(Operation::LDMI),
            0x23 => Some(Operation::LDMR),
            0x24 => Some(Operation::MOV),
            0x30 => Some(Operation::STMI),
            0x31 => Some(Operation::STMR),
            0x40 => Some(Operation::ADDI),
            0x41 => Some(Operation::ADDR2),
            0x42 => Some(Operation::ADDR3),
            0x50 => Some(Operation::SUBI),
            0x51 => Some(Operation::SUBR2),
            0x52 => Some(Operation::SUBR3),
            0x53 => Some(Operation::CMPI),
            0x54 => Some(Operation::CMPR),
            0x60 => Some(Operation::ANDI),
            0x61 => Some(Operation::ANDR2),
            0x62 => Some(Operation::ANDR3),
            0x63 => Some(Operation::TSTI),
            0x64 => Some(Operation::TSTR),
            0x70 => Some(Operation::ORI),
            0x71 => Some(Operation::ORR2),
            0x72 => Some(Operation::ORR3),
            0x80 => Some(Operation::XORI),
            0x81 => Some(Operation::XORR2),
            0x82 => Some(Operation::XORR3),
            0x90 => Some(Operation::MULI),
            0x91 => Some(Operation::MULR2),
            0x92 => Some(Operation::MULR3),
            0xA0 => Some(Operation::DIVI),
            0xA1 => Some(Operation::DIVR2),
            0xA2 => Some(Operation::DIVR3),
            0xA3 => Some(Operation::MODI),
            0xA4 => Some(Operation::MODR2),
            0xA5 => Some(Operation::MODR3),
            0xA6 => Some(Operation::REMI),
            0xA7 => Some(Operation::REMR2),
            0xA8 => Some(Operation::REMR3),
            0xB0 => Some(Operation::SHLN),
            0xB1 => Some(Operation::SHRN),
            0xB2 => Some(Operation::SARN),
            0xB3 => Some(Operation::SHLR),
            0xB4 => Some(Operation::SHRR),
            0xB5 => Some(Operation::SARR),
            0xC0 => Some(Operation::PUSH),
            0xC1 => Some(Operation::POP),
            0xC2 => Some(Operation::PUSHALL),
            0xC3 => Some(Operation::POPALL),
            0xC4 => Some(Operation::PUSHF),
            0xC5 => Some(Operation::POPF),
            0xD0 => Some(Operation::PALI),
            0xD1 => Some(Operation::PALR),
            0xE0 => Some(Operation::NOTI),
            0xE1 => Some(Operation::NOTR1),
            0xE2 => Some(Operation::NOTR2),
            0xE3 => Some(Operation::NEGI),
            0xE4 => Some(Operation::NEGR1),
            0xE5 => Some(Operation::NEGR2),
            _ => None,
        }
    }

    pub fn decode_condition(&self) -> Option<Condition> {
        match self.x() {
            0x0 => Some(Condition::Z),
            _ => None,
        }
    }

    // HH LL 00 00
    extract_argument!(hhll, u16, 16, 1);

    // HH LL 00 II
    extract_argument!(hh, u8, 8, 3);
    extract_argument!(ll, u8, 8, 2);
    extract_argument!(ii, u8, 8, 0);

    // 00 0Z YX 00
    extract_argument!(x, u8, 4, 2);
    extract_argument!(y, u8, 4, 3);
    extract_argument!(z, u8, 4, 4);

    // 00 0N 00 00
    extract_argument!(n, u8, 4, 4);

    // VT SR AD 00
    extract_argument!(v, u8, 4, 7);
    extract_argument!(t, u8, 4, 6);
    extract_argument!(s, u8, 4, 5);
    extract_argument!(r, u8, 4, 4);
    extract_argument!(a, u8, 4, 3);
    extract_argument!(d, u8, 4, 2);
}
