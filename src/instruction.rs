// TODO: Add documentation for each instruction.
// TODO: Implement fmt method for branch and instruction enums.

use std::fmt;

pub enum Branch {
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

pub enum Instruction {
    NOP,
    CLS,
    VBLNK,
    BGC { n: u8 },
    SPR { ll: u8, hh: u8 },
    DRWI { y: u8, x: u8, ll: u8, hh: u8 },
    DRWR { y: u8, x: u8, z: u8 },
    RND { x: u8, ll: u8, hh: u8 },
    FLIP { fh: bool, fv: bool },
    SND0,
    SND1 { ll: u8, hh: u8 },
    SND2 { ll: u8, hh: u8 },
    SND3 { ll: u8, hh: u8 },
    SNP { x: u8, hh: u8, ll: u8 },
    SNG { ad: u8, sr: u8, vt: u8 },
    JMPI { ll: u8, hh: u8 },
    JMC { ll: u8, hh: u8 },
    JX { x: u8, hh: u8, ll: u8 },
    JME { y: u8, x: u8, ll: u8, hh: u8 },
    CALLI { ll: u8, hh: u8 },
    RET,
    JMPR { x: u8 },
    CX { x: u8, ll: u8, hh: u8 },
    CALLR { x: u8 },
    LDIR { x: u8, ll: u8, hh: u8 },
    LDIS { ll: u8, hh: u8 },
    LDMI { x: u8, ll: u8, hh: u8 },
    LDMR { y: u8, x: u8 },
    MOV { y: u8, x: u8 },
    STMI { x: u8, ll: u8, hh: u8 },
    STMR { y: u8, x: u8 },
    ADDI { x: u8, ll: u8, hh: u8 },
    ADDR2 { y: u8, x: u8 },
    ADDR3 { y: u8, x: u8, z: u8 },
    SUBI { x: u8, ll: u8, hh: u8 },
    SUBR2 { y: u8, x: u8 },
    SUBR3 { y: u8, x: u8, z: u8 },
    CMPI { x: u8, ll: u8, hh: u8 },
    CMPR { y: u8, x: u8 },
    ANDI { x: u8, ll: u8, hh: u8 },
    ANDR2 { y: u8, x: u8 },
    ANDR3 { y: u8, x: u8, z: u8 },
    TSTI { x: u8, ll: u8, hh: u8 },
    TST { y: u8, x: u8 },
    ORI { x: u8, ll: u8, hh: u8 },
    ORR2 { y: u8, x: u8 },
    ORR3 { y: u8, x: u8, z: u8 },
    XORI { x: u8, ll: u8, hh: u8 },
    XORR2 { y: u8, x: u8 },
    XORR3 { y: u8, x: u8, z: u8 },
    MULI { x: u8, ll: u8, hh: u8 },
    MULR2 { y: u8, x: u8 },
    MULR3 { y: u8, x: u8, z: u8 },
    DIVI { x: u8, ll: u8, hh: u8 },
    DIVR2 { y: u8, x: u8 },
    DIVR3 { y: u8, x: u8, z: u8 },
    MODI { x: u8, ll: u8, hh: u8 },
    MODR2 { y: u8, x: u8 },
    MODR3 { y: u8, x: u8, z: u8 },
    REMI { x: u8, ll: u8, hh: u8 },
    REMR2 { y: u8, x: u8 },
    REMR3 { y: u8, x: u8, z: u8 },
    SHLN { x: u8, n: u8 },
    SHRN { x: u8, n: u8 },
    SARN { x: u8, n: u8 },
    SHLR { y: u8, x: u8 },
    SHRR { y: u8, x: u8 },
    SARR { y: u8, x: u8 },
    PUSH { x: u8 },
    POP { x: u8 },
    PUSHALL,
    POPALL,
    PUSHF,
    POPF,
    PALI { ll: u8, hh: u8 },
    PALR { x: u8 },
    NOTI { ll: u8, hh: u8 },
    NOTR1 { x: u8 },
    NOTR2 { y: u8, x: u8 },
    NEGI { x: u8, ll: u8, hh: u8 },
    NEGR1 { x: u8 },
    NEGR2 { y: u8, x: u8 },
}

impl Instruction {
    pub fn from_opcode(opcode: u32) -> Result<Instruction, &'static str> {
        // Deconstrct opcode into bytes and nibbles.
        let (b3, b2, b1, b0) = Instruction::opcode_to_bytes(opcode);
        let (_, _, n5, n4, n3, _, _, n0) = Instruction::opcode_to_nibbles(opcode);

        // Bind instruction abbreviations.
        // hh: 00 00 00 HH
        let hh = b0;
        // vt: 00 00 00 VT
        let vt = b0;
        // ll: 00 00 LL 00
        let ll = b1;
        // sr: 00 00 SR 00
        let sr = b1;
        // ad: 00 AD 00 00
        let ad = b2;
        // n:  00 00 N0 00
        let n = n3;
        // z:  00 00 Z0 00
        let z = n3;
        // c:  00 0x 00 00
        let c = n3;
        // x:  00 0X 00 00
        let x = n4;
        // y:  00 Y0 00 00
        let y = n5;

        // Decode the opcode into an instruction.
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match b3 {
            0x00 => Ok(Instruction::NOP),
            0x01 => Ok(Instruction::CLS),
            0x02 => Ok(Instruction::VBLNK),
            0x03 => Ok(Instruction::BGC { n }),
            0x04 => Ok(Instruction::SPR { ll, hh }),
            0x05 => Ok(Instruction::DRWI { y, x, ll, hh }),
            0x06 => Ok(Instruction::DRWR { y, x, z }),
            0x07 => Ok(Instruction::RND { x, ll, hh }),
            0x08 if n0 == 0 => Ok(Instruction::FLIP { fh: false, fv: false }),
            0x08 if n0 == 1 => Ok(Instruction::FLIP { fh: false, fv: true }),
            0x08 if n0 == 2 => Ok(Instruction::FLIP { fh: true, fv: false }),
            0x08 if n0 == 3 => Ok(Instruction::FLIP { fh: true, fv: true }),
            0x09 => Ok(Instruction::SND0),
            0x0A => Ok(Instruction::SND1 { ll, hh }),
            0x0B => Ok(Instruction::SND2 { ll, hh }),
            0x0C => Ok(Instruction::SND3 { ll, hh }),
            0x0D => Ok(Instruction::SNP { x, ll, hh }),
            0x0E => Ok(Instruction::SNG { ad, sr, vt }),
            0x10 => Ok(Instruction::JMPI { ll, hh }),
            0x11 => Ok(Instruction::JMC { ll, hh }),
            0x12 => Ok(Instruction::JX { x, ll, hh }),
            0x13 => Ok(Instruction::JME { y, x, ll, hh }),
            0x14 => Ok(Instruction::CALLI { ll, hh }),
            0x15 => Ok(Instruction::RET),
            0x16 => Ok(Instruction::JMPR { x }),
            0x17 => Ok(Instruction::CX { x, ll, hh }),
            0x18 => Ok(Instruction::CALLR { x }),
            0x20 => Ok(Instruction::LDIR { x, ll, hh }),
            0x21 => Ok(Instruction::LDIS { ll, hh }),
            0x22 => Ok(Instruction::LDMI { x, ll, hh }),
            0x23 => Ok(Instruction::LDMR { y, x }),
            0x24 => Ok(Instruction::MOV { y, x }),
            0x30 => Ok(Instruction::STMI { x, ll, hh }),
            0x31 => Ok(Instruction::STMR { y, x }),
            0x40 => Ok(Instruction::ADDI { x, ll, hh }),
            0x41 => Ok(Instruction::ADDR2 { y, x }),
            0x42 => Ok(Instruction::ADDR3 { y, x, z }),
            0x50 => Ok(Instruction::SUBI { x, ll, hh }),
            0x51 => Ok(Instruction::SUBR2 { y, x }),
            0x52 => Ok(Instruction::SUBR3 { y, x, z }),
            0x53 => Ok(Instruction::CMPI { x, ll, hh }),
            0x54 => Ok(Instruction::CMPR { y, x }),
            0x60 => Ok(Instruction::ANDI { x, ll, hh }),
            0x61 => Ok(Instruction::ANDR2 { y, x }),
            0x62 => Ok(Instruction::ANDR3 { y, x, z }),
            0x63 => Ok(Instruction::TSTI { x, ll, hh }),
            0x64 => Ok(Instruction::TST { y, x }),
            0x70 => Ok(Instruction::ORI { x, ll, hh }),
            0x71 => Ok(Instruction::ORR2 { y, x }),
            0x72 => Ok(Instruction::ORR3 { y, x, z }),
            0x80 => Ok(Instruction::XORI { x, ll, hh }),
            0x81 => Ok(Instruction::XORR2 { y, x }),
            0x82 => Ok(Instruction::XORR3 { y, x, z }),
            0x90 => Ok(Instruction::MULI { x, ll, hh }),
            0x91 => Ok(Instruction::MULR2 { y, x }),
            0x92 => Ok(Instruction::MULR3 { y, x, z }),
            0xA0 => Ok(Instruction::DIVI { x, ll, hh }),
            0xA1 => Ok(Instruction::DIVR2 { y, x }),
            0xA2 => Ok(Instruction::DIVR3 { y, x, z }),
            0xA3 => Ok(Instruction::MODI { x, ll, hh }),
            0xA4 => Ok(Instruction::MODR2 { y, x }),
            0xA5 => Ok(Instruction::MODR3 { y, x, z }),
            0xA6 => Ok(Instruction::REMI { x, ll, hh }),
            0xA7 => Ok(Instruction::REMR2 { y, x }),
            0xA8 => Ok(Instruction::REMR3 { y, x, z }),
            0xB0 => Ok(Instruction::SHLN { x, n }),
            0xB1 => Ok(Instruction::SHRN { x, n }),
            0xB2 => Ok(Instruction::SARN { x, n }),
            0xB3 => Ok(Instruction::SHLR { y, x }),
            0xB4 => Ok(Instruction::SHRR { y, x }),
            0xB5 => Ok(Instruction::SARR { y, x }),
            0xC0 => Ok(Instruction::PUSH { x }),
            0xC1 => Ok(Instruction::POP { x }),
            0xC2 => Ok(Instruction::PUSHALL),
            0xC3 => Ok(Instruction::POPALL),
            0xC4 => Ok(Instruction::PUSHF),
            0xC5 => Ok(Instruction::POPF),
            0xD0 => Ok(Instruction::PALI { ll, hh }),
            0xD1 => Ok(Instruction::PALR { x }),
            0xE0 => Ok(Instruction::NOTI { ll, hh }),
            0xE1 => Ok(Instruction::NOTR1 { x }),
            0xE2 => Ok(Instruction::NOTR2 { y, x }),
            0xE3 => Ok(Instruction::NEGI { x, ll, hh }),
            0xE4 => Ok(Instruction::NEGR1 { x }),
            0xE5 => Ok(Instruction::NEGR2 { y, x }),

            _ => Err(""),
        }
    }

    pub fn mnemonic(&self) -> &'static str {
        // TODO: Create the lookup table.
        ""
    }

    pub fn usage(&self) -> &'static str {
        // TODO: Create the lookup table.
        ""
    }

    pub fn version(&self) -> &'static str {
        // TODO: Create the lookup table.
        // NOTE: Define a Version struct instead?
        ""
    }

    fn opcode_to_bytes(opcode: u32) -> (u8, u8, u8, u8) {
        (
            (opcode & 0xFF000000 >> 0x18) as u8,
            (opcode & 0x00FF0000 >> 0x10) as u8,
            (opcode & 0x0000FF00 >> 0x08) as u8,
            (opcode & 0x000000FF) as u8,
        )
    }

    fn opcode_to_nibbles(opcode: u32) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        (
            (opcode & 0xF0000000 >> 0x1B) as u8,
            (opcode & 0x0F000000 >> 0x18) as u8,
            (opcode & 0x00F00000 >> 0x14) as u8,
            (opcode & 0x000F0000 >> 0x10) as u8,
            (opcode & 0x0000F000 >> 0x0B) as u8,
            (opcode & 0x00000F00 >> 0x08) as u8,
            (opcode & 0x000000F0 >> 0x04) as u8,
            (opcode & 0x0000000F) as u8,
        )
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Implement fmt for Instruction.
        Ok(())
    }
}
