use instruction::{Condition, Instruction};

#[derive(Default)]
struct Flags {
    c: bool,
    z: bool,
    o: bool,
    n: bool,
}

#[derive(Default)]
struct Controller {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    select: bool,
    start: bool,
    a: bool,
    b: bool,
}

#[derive(Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default)]
struct Cpu {
    m: Vec<u8>,
    r: [u16; 16],
    f: u8,
    pc: u16,
    sp: u16,

    g: Vec<u8>,
    p: [Color; 16],
    bg: u8,
    sh: u8,
    sw: u8,
    fh: bool,
    fv: bool,
}

trait Memory {
    fn read_u16(&self, index: usize) -> u16;
    fn write_u16(&mut self, index: usize, value: u16);
}

impl Memory for Vec<u8> {
    fn read_u16(&self, index: usize) -> u16 {
        (self[index] as u16) & ((self[index + 1] as u16) << 0x8)
    }

    fn write_u16(&mut self, index: usize, value: u16) {
        self[index] = (value & 0x00FF) as u8;
        self[index + 1] = (value & 0xFF00 >> 0x8) as u8;
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            m: Vec::with_capacity(65_536),
            g: Vec::with_capacity(320 * 240),
            ..Default::default()
        }
    }

    fn fetch(&self) -> u32 {
        0 as u32
    }

    fn step(&mut self) {
        let opcode = self.fetch();
        let instruction = Instruction::decode(opcode).expect("Instruction not found.");

        self.execute(&instruction);
    }

    fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::NOP => {}
            Instruction::CLS => {
                self.bg = 0u8;
                self.g.clear();
            }
            Instruction::VBLNK => {}
            Instruction::BGC { n } => {
                self.bg = n;
            }
            Instruction::SPR { ll, hh } => {
                self.sw = ll;
                self.sh = hh;
            }
            Instruction::DRWI { y, x, hhll } => {}
            Instruction::DRWR { y, x, z } => {}
            Instruction::RND { x, hhll } => {}
            Instruction::FLIP { fh, fv } => {
                self.fh = fh;
                self.fv = fv;
            }
            Instruction::SND0 {} => {}
            Instruction::SND1 { hhll } => {}
            Instruction::SND2 { hhll } => {}
            Instruction::SND3 { hhll } => {}
            Instruction::SNP { x, hhll } => {}
            Instruction::SNG { ad, sr, vt } => {}
            Instruction::JMPI { hhll } => {
                self.pc = hhll;
            }
            Instruction::JMC { hhll } => {}
            Instruction::JX { ref c, hhll } => {}
            Instruction::JME { y, x, hhll } => {
                if self.r[x as usize] == self.r[y as usize] {
                    self.pc = hhll;
                }
            }
            Instruction::CALLI { hhll } => {
                self.m.write_u16(self.sp as usize, self.pc);
                self.sp += 2;
                self.pc = hhll;
            }
            Instruction::RET => {
                self.sp -= 2;
                self.pc = self.m.read_u16(self.sp as usize);
            }
            Instruction::JMPR { x } => {
                self.pc = self.m.read_u16(x as usize);
            }
            Instruction::CX { ref c, hhll } => {}
            Instruction::CALLR { x } => {
                self.m.write_u16(self.sp as usize, self.pc);
                self.sp += 2;
                self.pc = self.r[x as usize];
            }
            Instruction::LDIR { x, hhll } => {
                self.r[x as usize] = hhll;
            }
            Instruction::LDIS { hhll } => {
                self.sp = hhll;
            }
            Instruction::LDMI { x, hhll } => {}
            Instruction::LDMR { y, x } => {
                self.r[x as usize] = self.m.read_u16(y as usize);
            }
            Instruction::MOV { y, x } => {
                self.r[x as usize] = self.r[y as usize];
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod test {}
