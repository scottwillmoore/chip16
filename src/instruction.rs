use self::Condition::*;
use self::Instruction::*;

struct Opcode(u32);

#[cfg_attr(rustfmt, rustfmt_skip)]
impl Opcode {
    // Words.
    // hhll = 00 00 HH LL
    #[inline(always)] fn hhll(&self) -> u16 { (self.0 & 0xFFFF) as u16 }

    // Bytes.
    // ii = II 00 00 00
    #[inline(always)] fn ii(&self) -> u8 { (self.0 & 0xFF000000 >> 24) as u8 }
    // hh = 00 00 HH 00
    #[inline(always)] fn hh(&self) -> u8 { (self.0 & 0xFF00 >> 8) as u8 }
    // ll = 00 00 00 LL
    #[inline(always)] fn ll(&self) -> u8 { (self.0 & 0xFF) as u8 }

    // Nibbles.
    // ad = 00 A0 00 00
    #[inline(always)] fn a(&self) -> u8 { (self.0 & 0xF00000 >> 20) as u8 }
    // ad = 00 0D 00 00
    #[inline(always)] fn d(&self) -> u8 { (self.0 & 0xF0000 >> 16) as u8 }
    // sr = 00 00 S0 00
    #[inline(always)] fn s(&self) -> u8 { (self.0 & 0xF000 >> 12) as u8 }
    // sr = 00 00 0R 00
    #[inline(always)] fn r(&self) -> u8 { (self.0 & 0xF00 >> 8) as u8 }
    // vt = 00 00 00 V0
    #[inline(always)] fn v(&self) -> u8 { (self.0 & 0xF0 >> 4) as u8 }
    // vt = 00 00 00 0T
    #[inline(always)] fn t(&self) -> u8 { (self.0 & 0xF) as u8 }
    // x = 00 0X 00 00
    #[inline(always)] fn x(&self) -> u8 { (self.0 & 0xF0000 >> 16) as u8 }
    // y = 00 Y0 00 00
    #[inline(always)] fn y(&self) -> u8 { (self.0 & 0xF00000 >> 20) as u8 }
    // z = 00 00 Z0 00
    #[inline(always)] fn z(&self) -> u8 { (self.0 & 0xF000 >> 12) as u8 }
    // n = 00 00 N0 00
    #[inline(always)] fn n(&self) -> u8 { (self.0 & 0xF000 >> 12) as u8 }
    // c = 00 0x 00 00
    #[inline(always)] fn c(&self) -> Condition { Condition::decode(self.x()).unwrap() }
}

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
    pub fn decode(x: u8) -> Result<Condition, &'static str> {
        match x {
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
            _ => Err("Failed to decode byte into a condition."),
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum Instruction {
    NOP,
    CLS,
    VBLNK,
    BGC { n: u8 },
    SPR { width: u8, height: u8 },
    DRWI { x: u8, y: u8, address: u16 },
    DRWR { x: u8, y: u8, z: u8 },
    RND { x: u8, address: u16 },
    FLIP { flip_horizontal: bool, flip_vertical: bool },
    SND0,
    SND1 { address: u16 },
    SND2 { address: u16 },
    SND3 { address: u16 },
    SNP { x: u8, address: u16 },
    SNG { attack: u8, decay: u8, sustain: u8, release: u8, volume: u8, wave: u8 },
    JMPI { address: u16 },
    JMC { address: u16 },
    JX { condition: Condition, address: u16 },
    JME { x: u8, y: u8, address: u16 },
    CALLI { address: u16 },
    RET,
    JMPR { x: u8 },
    CX { condition: Condition, address: u16 },
    CALLR { x: u8 },
    LDIR { x: u8, immediate: u16 },
    LDIS { immediate: u16 },
    LDMI { x: u8, address: u16 },
    LDMR { x: u8, y: u8 },
    MOV { x: u8, y: u8 },
    STMI { x: u8, address: u16 },
    STMR { x: u8, y: u8 },
    ADDI { x: u8, immediate: u16 },
    ADDR2 { x: u8, y: u8 },
    ADDR3 { x: u8, y: u8, z: u8 },
    SUBI { x: u8, immediate: u16 },
    SUBR2 { x: u8, y: u8 },
    SUBR3 { x: u8, y: u8, z: u8 },
    CMPI { x: u8, immediate: u16 },
    CMPR { x: u8, y: u8 },
    ANDI { x: u8, immediate: u16 },
    ANDR2 { x: u8, y: u8 },
    ANDR3 { x: u8, y: u8, z: u8 },
    TSTI { x: u8, immediate: u16 },
    TSTR { x: u8, y: u8 },
    ORI { x: u8, immediate: u16 },
    ORR2 { x: u8, y: u8 },
    ORR3 { x: u8, y: u8, z: u8 },
    XORI { x: u8, immediate: u16 },
    XORR2 { x: u8, y: u8 },
    XORR3 { x: u8, y: u8, z: u8 },
    MULI { x: u8, immediate: u16 },
    MULR2 { x: u8, y: u8 },
    MULR3 { x: u8, y: u8, z: u8 },
    DIVI { x: u8, immediate: u16 },
    DIVR2 { x: u8, y: u8 },
    DIVR3 { x: u8, y: u8, z: u8 },
    MODI { x: u8, immediate: u16 },
    MODR2 { x: u8, y: u8 },
    MODR3 { x: u8, y: u8, z: u8 },
    REMI { x: u8, immediate: u16 },
    REMR2 { x: u8, y: u8 },
    REMR3 { x: u8, y: u8, z: u8 },
    SHLN { x: u8, n: u8 },
    SHRN { x: u8, n: u8 },
    SARN { x: u8, n: u8 },
    SHLR { x: u8, y: u8 },
    SHRR { x: u8, y: u8 },
    SARR { x: u8, y: u8 },
    PUSH { x: u8 },
    POP { x: u8 },
    PUSHALL,
    POPALL,
    PUSHF,
    POPF,
    PALI { address: u16 },
    PALR { x: u8 },
    NOTI { x: u8, immediate: u16 },
    NOTR1 { x: u8 },
    NOTR2 { x: u8, y: u8 },
    NEGI { x: u8, immediate: u16 },
    NEGR1 { x: u8 },
    NEGR2 { x: u8, y: u8 },
}

impl Instruction {
    pub fn decode(opcode: u32) -> Result<Instruction, &'static str> {
        // Wrap opcode in struct with inline methods.
        let opcode = Opcode(opcode);

        // Decode the opcode into an instruction.
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match opcode.ii() {
            0x00 => Ok(NOP),
            0x01 => Ok(CLS),
            0x02 => Ok(VBLNK),
            0x03 => Ok(BGC { n: opcode.n() }),
            0x04 => Ok(SPR { width: opcode.ll(), height: opcode.hh() }),
            0x05 => Ok(DRWI { x: opcode.x(), y: opcode.y(), address: opcode.hhll() }),
            0x06 => Ok(DRWR { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0x07 => Ok(RND { x: opcode.x(), address: opcode.hhll() }),
            0x08 if opcode.n() == 0 => Ok(FLIP { flip_horizontal: false, flip_vertical: false }),
            0x08 if opcode.n() == 1 => Ok(FLIP { flip_horizontal: false, flip_vertical: true }),
            0x08 if opcode.n() == 2 => Ok(FLIP { flip_horizontal: true, flip_vertical: false }),
            0x08 if opcode.n() == 3 => Ok(FLIP { flip_horizontal: true, flip_vertical: true }),
            0x09 => Ok(SND0),
            0x0A => Ok(SND1 { address: opcode.hhll() }),
            0x0B => Ok(SND2 { address: opcode.hhll() }),
            0x0C => Ok(SND3 { address: opcode.hhll() }),
            0x0D => Ok(SNP { x: opcode.x(), address: opcode.hhll() }),
            0x0E => Ok(SNG { attack: opcode.a(), decay: opcode.d(), sustain: opcode.s(), release: opcode.r(), volume: opcode.v(), wave: opcode.t() }),
            0x10 => Ok(JMPI { address: opcode.hhll() }),
            0x11 => Ok(JMC { address: opcode.hhll() }),
            0x12 => Ok(JX { condition: opcode.c(), address: opcode.hhll() }),
            0x13 => Ok(JME { x: opcode.x(), y: opcode.y(), address: opcode.hhll() }),
            0x14 => Ok(CALLI { address: opcode.hhll() }),
            0x15 => Ok(RET),
            0x16 => Ok(JMPR { x: opcode.x() }),
            0x17 => Ok(CX { condition: opcode.c(), address: opcode.hhll() }),
            0x18 => Ok(CALLR { x: opcode.x() }),
            0x20 => Ok(LDIR { x: opcode.x(), immediate: opcode.hhll() }),
            0x21 => Ok(LDIS { immediate: opcode.hhll() }),
            0x22 => Ok(LDMI { x: opcode.x(), address: opcode.hhll() }),
            0x23 => Ok(LDMR { x: opcode.x(), y: opcode.y() }),
            0x24 => Ok(MOV { x: opcode.x(), y: opcode.y() }),
            0x30 => Ok(STMI { x: opcode.x(), address: opcode.hhll() }),
            0x31 => Ok(STMR { x: opcode.x(), y: opcode.y() }),
            0x40 => Ok(ADDI { x: opcode.x(), immediate: opcode.hhll() }),
            0x41 => Ok(ADDR2 { x: opcode.x(), y: opcode.y()  }),
            0x42 => Ok(ADDR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0x50 => Ok(SUBI { x: opcode.x(), immediate: opcode.hhll() }),
            0x51 => Ok(SUBR2 { x: opcode.x(), y: opcode.y() }),
            0x52 => Ok(SUBR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0x53 => Ok(CMPI { x: opcode.x(), immediate: opcode.hhll() }),
            0x54 => Ok(CMPR { x: opcode.x(), y: opcode.y() }),
            0x60 => Ok(ANDI { x: opcode.x(), immediate: opcode.hhll() }),
            0x61 => Ok(ANDR2 { x: opcode.x(), y: opcode.y() }),
            0x62 => Ok(ANDR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0x63 => Ok(TSTI { x: opcode.x(), immediate: opcode.hhll() }),
            0x64 => Ok(TSTR { x: opcode.x(), y: opcode.y() }),
            0x70 => Ok(ORI { x: opcode.x(), immediate: opcode.hhll() }),
            0x71 => Ok(ORR2 { x: opcode.x(), y: opcode.y() }),
            0x72 => Ok(ORR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0x80 => Ok(XORI { x: opcode.x(), immediate: opcode.hhll() }),
            0x81 => Ok(XORR2 { x: opcode.x(), y: opcode.y() }),
            0x82 => Ok(XORR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0x90 => Ok(MULI { x: opcode.x(), immediate: opcode.hhll() }),
            0x91 => Ok(MULR2 { x: opcode.x(), y: opcode.y() }),
            0x92 => Ok(MULR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0xA0 => Ok(DIVI { x: opcode.x(), immediate: opcode.hhll() }),
            0xA1 => Ok(DIVR2 { x: opcode.x(), y: opcode.y() }),
            0xA2 => Ok(DIVR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0xA3 => Ok(MODI { x: opcode.x(), immediate: opcode.hhll() }),
            0xA4 => Ok(MODR2 { x: opcode.x(), y: opcode.y() }),
            0xA5 => Ok(MODR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0xA6 => Ok(REMI { x: opcode.x(), immediate: opcode.hhll() }),
            0xA7 => Ok(REMR2 { x: opcode.x(), y: opcode.y() }),
            0xA8 => Ok(REMR3 { x: opcode.x(), y: opcode.y(), z: opcode.z() }),
            0xB0 => Ok(SHLN { x: opcode.x(), n: opcode.n() }),
            0xB1 => Ok(SHRN { x: opcode.x(), n: opcode.n() }),
            0xB2 => Ok(SARN { x: opcode.x(), n: opcode.n() }),
            0xB3 => Ok(SHLR { x: opcode.x(), y: opcode.y() }),
            0xB4 => Ok(SHRR { x: opcode.x(), y: opcode.y() }),
            0xB5 => Ok(SARR { x: opcode.x(), y: opcode.y() }),
            0xC0 => Ok(PUSH { x: opcode.x() }),
            0xC1 => Ok(POP { x: opcode.x() }),
            0xC2 => Ok(PUSHALL),
            0xC3 => Ok(POPALL),
            0xC4 => Ok(PUSHF),
            0xC5 => Ok(POPF),
            0xD0 => Ok(PALI { address: opcode.hhll() }),
            0xD1 => Ok(PALR { x: opcode.x() }),
            0xE0 => Ok(NOTI { x: opcode.x(), immediate: opcode.hhll() }),
            0xE1 => Ok(NOTR1 { x: opcode.x() }),
            0xE2 => Ok(NOTR2 { x: opcode.x(), y: opcode.y() }),
            0xE3 => Ok(NEGI { x: opcode.x(), immediate: opcode.hhll() }),
            0xE4 => Ok(NEGR1 { x: opcode.x() }),
            0xE5 => Ok(NEGR2 { x: opcode.x(), y: opcode.y() }),
            _ => Err("Failed to decode opcode into an instruction."),
        }
    }

    pub fn disassemble(&self) -> String {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match self {
            // TODO: Include instruction arguments in disassembled string.
            NOP => format!("NOP"),
            CLS => format!("CLS"),
            VBLNK => format!("VBLNK"),
            BGC { n } => format!("BGC {:01x}", n),
            SPR { width, height } => format!("SPR"),
            DRWI { x, y, address } => format!("DRWI {:01x}, {:01x}, {:04x}", x, y, address),
            DRWR { x, y, z } => format!("DRWR {:01x}, {:01x}, {:01x}", x, y, z),
            RND { x, address } => format!("RND {:01x}, {:04x}", x, address),
            FLIP { flip_horizontal, flip_vertical } => format!("FLIP {}, {}", *flip_horizontal as u8, *flip_vertical as u8),
            SND0 => format!("SND0"),
            SND1 { address } => format!("SND1 {:04x}", address),
            SND2 { address } => format!("SND2 {:04x}", address),
            SND3 { address } => format!("SND3 {:04x}", address),
            SNP { x, address } => format!("SNP"),
            SNG { attack, decay, sustain, release, volume, wave } => format!("SNG"),
            JMPI { address } => format!("JMPI"),
            JMC { address } => format!("JMC"),
            JX { condition, address } => format!("JX"),
            JME { x, y, address } => format!("JME"),
            CALLI { address } => format!("CALLI"),
            RET => format!("RET"),
            JMPR { x } => format!("JMPR"),
            CX { condition, address } => format!("CX"),
            CALLR { x } => format!("CALLR"),
            LDIR { x, immediate } => format!("LDIR"),
            LDIS { immediate } => format!("LDIS"),
            LDMI { x, address } => format!("LDMI"),
            LDMR { x, y } => format!("LDMR"),
            MOV { x, y } => format!("MOV"),
            STMI { x, address } => format!("STMI"),
            STMR { x, y } => format!("STMR"),
            ADDI { x, immediate } => format!("ADDI"),
            ADDR2 { x, y } => format!("ADDR2"),
            ADDR3 { x, y, z } => format!("ADDR3"),
            SUBI { x, immediate } => format!("SUBI"),
            SUBR2 { x, y } => format!("SUBR2"),
            SUBR3 { x, y, z } => format!("SUBR3"),
            CMPI { x, immediate } => format!("CMPI"),
            CMPR { x, y } => format!("CMPR"),
            ANDI { x, immediate } => format!("ANDI"),
            ANDR2 { x, y } => format!("ANDR2"),
            ANDR3 { x, y, z } => format!("ANDR3"),
            TSTI { x, immediate } => format!("TSTI"),
            TSTR { x, y } => format!("TSTR"),
            ORI { x, immediate } => format!("ORI"),
            ORR2 { x, y } => format!("ORR2"),
            ORR3 { x, y, z } => format!("ORR3"),
            XORI { x, immediate } => format!("XORI"),
            XORR2 { x, y } => format!("XORR2"),
            XORR3 { x, y, z } => format!("XORR3"),
            MULI { x, immediate } => format!("MULI"),
            MULR2 { x, y } => format!("MULR2"),
            MULR3 { x, y, z } => format!("MULR3"),
            DIVI { x, immediate } => format!("DIVI"),
            DIVR2 { x, y } => format!("DIVR2"),
            DIVR3 { x, y, z } => format!("DIVR3"),
            MODI { x, immediate } => format!("MODI"),
            MODR2 { x, y } => format!("MODR2"),
            MODR3 { x, y, z } => format!("MODR3"),
            REMI { x, immediate } => format!("REMI"),
            REMR2 { x, y } => format!("REMR2"),
            REMR3 { x, y, z } => format!("REMR3"),
            SHLN { x, n } => format!("SHLN"),
            SHRN { x, n } => format!("SHRN"),
            SARN { x, n } => format!("SARN"),
            SHLR { x, y } => format!("SHLR"),
            SHRR { x, y } => format!("SHRR"),
            SARR { x, y } => format!("SARR"),
            PUSH { x } => format!("PUSH"),
            POP { x } => format!("POP"),
            PUSHALL => format!("PUSHALL"),
            POPALL => format!("POPALL"),
            PUSHF => format!("PUSHF"),
            POPF => format!("POPF"),
            PALI { address } => format!("PALI"),
            PALR { x } => format!("PALR"),
            NOTI { x, immediate } => format!("NOTI"),
            NOTR1 { x } => format!("NOTR1"),
            NOTR2 { x, y } => format!("NOTR2"),
            NEGI { x, immediate } => format!("NEGI"),
            NEGR1 { x } => format!("NEGR1"),
            NEGR2 { x, y } => format!("NEGR2"),
        }
    }
}
