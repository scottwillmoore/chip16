use condition::Condition;
use flags::Flags;
use instruction::Instruction;

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
    f: Flags,
    pc: u16,
    sp: u16,

    v: Vec<u8>,
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
            v: Vec::with_capacity(320 * 240),
            ..Default::default()
        }
    }

    fn step(&mut self) {
        let instruction = self.fetch();

        self.execute(&instruction);
    }

    fn fetch(&self) -> Instruction {
        Instruction::decode(0).unwrap()
    }

    fn test(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Z => self.f.z,
            Condition::NZ => !self.f.z,
            Condition::N => self.f.n,
            Condition::NN => !self.f.n,
            Condition::P => !self.f.n && !self.f.z,
            Condition::O => self.f.o,
            Condition::NO => !self.f.o,
            Condition::A => !self.f.c && !self.f.z,
            Condition::AE => !self.f.c,
            Condition::B => self.f.c,
            Condition::BE => self.f.c || self.f.z,
            Condition::G => self.f.o == self.f.n && !self.f.z,
            Condition::GE => self.f.o == self.f.n,
            Condition::L => self.f.o != self.f.n,
            Condition::LE => self.f.o != self.f.n || self.f.z,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::NOP => {}
            Instruction::CLS => {
                self.bg = 0u8;
                self.v.clear();
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
            Instruction::JMC { hhll } => {
                if self.f.c {
                    self.pc = hhll;
                }
            }
            Instruction::JX { ref c, hhll } => {
                if self.test(c) {
                    self.pc = hhll;
                }
            }
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
                self.pc = self.r[x as usize];
            }
            Instruction::CX { ref c, hhll } => {
                if self.test(c) {
                    self.m.write_u16(self.sp as usize, self.pc);
                    self.sp += 2;
                    self.pc = hhll;
                }
            }
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
            Instruction::LDMI { x, hhll } => {
                self.r[x as usize] = hhll;
            }
            Instruction::LDMR { y, x } => {
                self.r[x as usize] = self.m.read_u16(self.r[y as usize] as usize);
            }
            Instruction::MOV { y, x } => {
                self.r[x as usize] = self.r[y as usize];
            }
            Instruction::STMI { x, hhll } => {
                self.m.write_u16(hhll as usize, self.r[x as usize]);
            }
            Instruction::STMR { y, x } => {
                self.m
                    .write_u16(self.r[y as usize] as usize, self.r[x as usize]);
            }
            Instruction::ADDI { x, hhll } => {
                // TODO: Handle flags!

                self.r[x as usize] = self.r[x as usize] + hhll;
            }
            Instruction::ADDR2 { y, x } => {}
            Instruction::ADDR3 { y, x, z } => {}
            Instruction::SUBI { x, hhll } => {}
            Instruction::SUBR2 { y, x } => {}
            Instruction::SUBR3 { y, x, z } => {}
            Instruction::CMPI { x, hhll } => {}
            Instruction::CMPR { y, x } => {}
            Instruction::ANDI { x, hhll } => {}
            Instruction::ANDR2 { y, x } => {}
            Instruction::ANDR3 { y, x, z } => {}
            Instruction::TSTI { x, hhll } => {}
            Instruction::TST { y, x } => {}
            Instruction::ORI { x, hhll } => {}
            Instruction::ORR2 { y, x } => {}
            Instruction::ORR3 { y, x, z } => {}
            Instruction::XORI { x, hhll } => {}
            Instruction::XORR2 { y, x } => {}
            Instruction::XORR3 { y, x, z } => {}
            Instruction::MULI { x, hhll } => {}
            Instruction::MULR2 { y, x } => {}
            Instruction::MULR3 { y, x, z } => {}
            Instruction::DIVI { x, hhll } => {}
            Instruction::DIVR2 { y, x } => {}
            Instruction::DIVR3 { y, x, z } => {}
            Instruction::MODI { x, hhll } => {}
            Instruction::MODR2 { y, x } => {}
            Instruction::MODR3 { y, x, z } => {}
            Instruction::REMI { x, hhll } => {}
            Instruction::REMR2 { y, x } => {}
            Instruction::REMR3 { y, x, z } => {}
            Instruction::SHLN { x, n } => {}
            Instruction::SHRN { x, n } => {}
            Instruction::SARN { x, n } => {}
            Instruction::SHLR { y, x } => {}
            Instruction::SHRR { y, x } => {}
            Instruction::SARR { y, x } => {}
            Instruction::PUSH { x } => {
                self.m.write_u16(self.sp as usize, self.r[x as usize]);
                self.sp += 2;
            }
            Instruction::POP { x } => {
                self.sp -= 2;
                self.r[x as usize] = self.m.read_u16(self.sp as usize);
            }
            Instruction::PUSHALL => {
                for r in self.r.iter() {
                    self.m.write_u16(self.sp as usize, *r);
                    self.sp += 2;
                }
            }
            Instruction::POPALL => {
                for r in self.r.iter_mut().rev() {
                    *r = self.m.read_u16(self.sp as usize);
                    self.sp -= 2;
                }
            }
            Instruction::PUSHF => {
                self.m[self.sp as usize] = From::from(&self.f);
                self.sp += 2;
            }
            Instruction::POPF => {
                self.sp -= 2;
                self.f = From::from(self.m[self.sp as usize]);
            }
            Instruction::PALI { hhll } => {
                for p in self.p.iter_mut() {
                    p.r = self.m[self.sp as usize];
                    p.g = self.m[(self.sp + 1) as usize];
                    p.b = self.m[(self.sp + 2) as usize];
                    self.sp += 3;
                }
            }
            Instruction::PALR { x } => {
                let mut i = self.r[x as usize];
                for p in self.p.iter_mut() {
                    p.r = self.m[i as usize];
                    p.g = self.m[(i + 1) as usize];
                    p.b = self.m[(i + 2) as usize];
                    i += 3;
                }
            }
            Instruction::NOTI { hhll } => {}
            Instruction::NOTR1 { x } => {}
            Instruction::NOTR2 { y, x } => {}
            Instruction::NEGI { x, hhll } => {}
            Instruction::NEGR1 { x } => {}
            Instruction::NEGR2 { y, x } => {}
            _ => {}
        };
    }
}
