use std::fmt;

use self::Instruction::*;
use condition::Condition;

pub enum Instruction {
    NOP,
    CLS,
    VBLNK,
    BGC { n: u8 },
    SPR { ll: u8, hh: u8 },
    DRWI { y: u8, x: u8, hhll: u16 },
    DRWR { y: u8, x: u8, z: u8 },
    RND { x: u8, hhll: u16 },
    FLIP { fh: bool, fv: bool },
    SND0,
    SND1 { hhll: u16 },
    SND2 { hhll: u16 },
    SND3 { hhll: u16 },
    SNP { x: u8, hhll: u16 },
    SNG { ad: u8, sr: u8, vt: u8 },
    JMPI { hhll: u16 },
    JMC { hhll: u16 },
    JX { c: Condition, hhll: u16 },
    JME { y: u8, x: u8, hhll: u16 },
    CALLI { hhll: u16 },
    RET,
    JMPR { x: u8 },
    CX { c: Condition, hhll: u16 },
    CALLR { x: u8 },
    LDIR { x: u8, hhll: u16 },
    LDIS { hhll: u16 },
    LDMI { x: u8, hhll: u16 },
    LDMR { y: u8, x: u8 },
    MOV { y: u8, x: u8 },
    STMI { x: u8, hhll: u16 },
    STMR { y: u8, x: u8 },
    ADDI { x: u8, hhll: u16 },
    ADDR2 { y: u8, x: u8 },
    ADDR3 { y: u8, x: u8, z: u8 },
    SUBI { x: u8, hhll: u16 },
    SUBR2 { y: u8, x: u8 },
    SUBR3 { y: u8, x: u8, z: u8 },
    CMPI { x: u8, hhll: u16 },
    CMPR { y: u8, x: u8 },
    ANDI { x: u8, hhll: u16 },
    ANDR2 { y: u8, x: u8 },
    ANDR3 { y: u8, x: u8, z: u8 },
    TSTI { x: u8, hhll: u16 },
    TST { y: u8, x: u8 },
    ORI { x: u8, hhll: u16 },
    ORR2 { y: u8, x: u8 },
    ORR3 { y: u8, x: u8, z: u8 },
    XORI { x: u8, hhll: u16 },
    XORR2 { y: u8, x: u8 },
    XORR3 { y: u8, x: u8, z: u8 },
    MULI { x: u8, hhll: u16 },
    MULR2 { y: u8, x: u8 },
    MULR3 { y: u8, x: u8, z: u8 },
    DIVI { x: u8, hhll: u16 },
    DIVR2 { y: u8, x: u8 },
    DIVR3 { y: u8, x: u8, z: u8 },
    MODI { x: u8, hhll: u16 },
    MODR2 { y: u8, x: u8 },
    MODR3 { y: u8, x: u8, z: u8 },
    REMI { x: u8, hhll: u16 },
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
    PALI { hhll: u16 },
    PALR { x: u8 },
    NOTI { x: u8, hhll: u16 },
    NOTR1 { x: u8 },
    NOTR2 { y: u8, x: u8 },
    NEGI { x: u8, hhll: u16 },
    NEGR1 { x: u8 },
    NEGR2 { y: u8, x: u8 },
}

impl Instruction {
    pub fn decode(opcode: u32) -> Result<Instruction, &'static str> {
        // Deconstrct opcode into bytes and nibbles.
        let (_, w0) = Instruction::opcode_to_words(opcode);
        let (b3, b2, b1, b0) = Instruction::opcode_to_bytes(opcode);
        let (_, _, n5, n4, n3, _, _, n0) = Instruction::opcode_to_nibbles(opcode);

        // Bind instruction abbreviations.
        // hhll: 00 00 HH LL
        let hhll = w0;
        // hh: 00 00 00 LL
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
        // x:  00 0X 00 00
        let x = n4;
        // y:  00 Y0 00 00
        let y = n5;
        // c:  00 0x 00 00
        let c = Condition::decode(x).unwrap();

        // Decode the opcode into an instruction.
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match b3 {
            0x00 => Ok(NOP),
            0x01 => Ok(CLS),
            0x02 => Ok(VBLNK),
            0x03 => Ok(BGC { n }),
            0x04 => Ok(SPR { ll, hh }),
            0x05 => Ok(DRWI { y, x, hhll }),
            0x06 => Ok(DRWR { y, x, z }),
            0x07 => Ok(RND { x, hhll }),
            0x08 if n0 == 0 => Ok(FLIP { fh: false, fv: false }),
            0x08 if n0 == 1 => Ok(FLIP { fh: false, fv: true }),
            0x08 if n0 == 2 => Ok(FLIP { fh: true, fv: false }),
            0x08 if n0 == 3 => Ok(FLIP { fh: true, fv: true }),
            0x09 => Ok(SND0),
            0x0A => Ok(SND1 { hhll }),
            0x0B => Ok(SND2 { hhll }),
            0x0C => Ok(SND3 { hhll }),
            0x0D => Ok(SNP { x, hhll }),
            0x0E => Ok(SNG { ad, sr, vt }),
            0x10 => Ok(JMPI { hhll }),
            0x11 => Ok(JMC { hhll }),
            0x12 => Ok(JX { c, hhll }),
            0x13 => Ok(JME { y, x, hhll }),
            0x14 => Ok(CALLI { hhll }),
            0x15 => Ok(RET),
            0x16 => Ok(JMPR { x }),
            0x17 => Ok(CX { c, hhll }),
            0x18 => Ok(CALLR { x }),
            0x20 => Ok(LDIR { x, hhll }),
            0x21 => Ok(LDIS { hhll }),
            0x22 => Ok(LDMI { x, hhll }),
            0x23 => Ok(LDMR { y, x }),
            0x24 => Ok(MOV { y, x }),
            0x30 => Ok(STMI { x, hhll }),
            0x31 => Ok(STMR { y, x }),
            0x40 => Ok(ADDI { x, hhll }),
            0x41 => Ok(ADDR2 { y, x }),
            0x42 => Ok(ADDR3 { y, x, z }),
            0x50 => Ok(SUBI { x, hhll }),
            0x51 => Ok(SUBR2 { y, x }),
            0x52 => Ok(SUBR3 { y, x, z }),
            0x53 => Ok(CMPI { x, hhll }),
            0x54 => Ok(CMPR { y, x }),
            0x60 => Ok(ANDI { x, hhll }),
            0x61 => Ok(ANDR2 { y, x }),
            0x62 => Ok(ANDR3 { y, x, z }),
            0x63 => Ok(TSTI { x, hhll }),
            0x64 => Ok(TST { y, x }),
            0x70 => Ok(ORI { x, hhll }),
            0x71 => Ok(ORR2 { y, x }),
            0x72 => Ok(ORR3 { y, x, z }),
            0x80 => Ok(XORI { x, hhll }),
            0x81 => Ok(XORR2 { y, x }),
            0x82 => Ok(XORR3 { y, x, z }),
            0x90 => Ok(MULI { x, hhll }),
            0x91 => Ok(MULR2 { y, x }),
            0x92 => Ok(MULR3 { y, x, z }),
            0xA0 => Ok(DIVI { x, hhll }),
            0xA1 => Ok(DIVR2 { y, x }),
            0xA2 => Ok(DIVR3 { y, x, z }),
            0xA3 => Ok(MODI { x, hhll }),
            0xA4 => Ok(MODR2 { y, x }),
            0xA5 => Ok(MODR3 { y, x, z }),
            0xA6 => Ok(REMI { x, hhll }),
            0xA7 => Ok(REMR2 { y, x }),
            0xA8 => Ok(REMR3 { y, x, z }),
            0xB0 => Ok(SHLN { x, n }),
            0xB1 => Ok(SHRN { x, n }),
            0xB2 => Ok(SARN { x, n }),
            0xB3 => Ok(SHLR { y, x }),
            0xB4 => Ok(SHRR { y, x }),
            0xB5 => Ok(SARR { y, x }),
            0xC0 => Ok(PUSH { x }),
            0xC1 => Ok(POP { x }),
            0xC2 => Ok(PUSHALL),
            0xC3 => Ok(POPALL),
            0xC4 => Ok(PUSHF),
            0xC5 => Ok(POPF),
            0xD0 => Ok(PALI { hhll }),
            0xD1 => Ok(PALR { x }),
            0xE0 => Ok(NOTI { x, hhll }),
            0xE1 => Ok(NOTR1 { x }),
            0xE2 => Ok(NOTR2 { y, x }),
            0xE3 => Ok(NEGI { x, hhll }),
            0xE4 => Ok(NEGR1 { x }),
            0xE5 => Ok(NEGR2 { y, x }),
            _ => Err("Failed to decode opcode into an instruction."),
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

    fn opcode_to_words(opcode: u32) -> (u16, u16) {
        (
            (opcode & 0xFFFF0000 >> 0x10) as u16,
            (opcode & 0x0000FFFF) as u16,
        )
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
