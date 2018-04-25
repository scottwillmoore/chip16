use rand;

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
    memory: Vec<u8>,
    registers: [u16; 16],
    flags: Flags,
    program_counter: u16,
    stack_pointer: u16,
    video_memory: Vec<u8>,
    palette: [Color; 16],
    background: u8,
    sprite_height: u8,
    sprite_width: u8,
    flip_horizontal: bool,
    flip_vertical: bool,
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
    pub fn new() -> Cpu {
        Cpu {
            memory: Vec::with_capacity(65_536),
            video_memory: Vec::with_capacity(320 * 240),
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();

        self.execute(&instruction);
    }

    fn fetch(&self) -> Instruction {
        Instruction::decode(0).unwrap()
    }

    fn test(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Z => self.flags.zero,
            Condition::NZ => !self.flags.zero,
            Condition::N => self.flags.negative,
            Condition::NN => !self.flags.negative,
            Condition::P => !self.flags.negative && !self.flags.zero,
            Condition::O => self.flags.overflow,
            Condition::NO => !self.flags.overflow,
            Condition::A => !self.flags.carry && !self.flags.zero,
            Condition::AE => !self.flags.carry,
            Condition::B => self.flags.carry,
            Condition::BE => self.flags.carry || self.flags.zero,
            Condition::G => self.flags.overflow == self.flags.negative && !self.flags.zero,
            Condition::GE => self.flags.overflow == self.flags.negative,
            Condition::L => self.flags.overflow != self.flags.negative,
            Condition::LE => self.flags.overflow != self.flags.negative || self.flags.zero,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::NOP => {}
            Instruction::CLS => {
                self.background = 0u8;
                self.video_memory.clear();
            }
            // Instruction::VBLNK => {}
            Instruction::BGC { n } => {
                self.background = n;
            }
            Instruction::SPR { ll, hh } => {
                self.sprite_width = ll;
                self.sprite_height = hh;
            }
            // Instruction::DRWI { y, x, hhll } => {}
            // Instruction::DRWR { y, x, z } => {}
            // Instruction::RND { x, hhll } => {
            //     self.registers[x as usize] = rand::random::<u16>() % hhll;
            // }
            Instruction::FLIP { fh, fv } => {
                self.flip_horizontal = fh;
                self.flip_vertical = fv;
            }
            // Instruction::SND0 {} => {}
            // Instruction::SND1 { hhll } => {}
            // Instruction::SND2 { hhll } => {}
            // Instruction::SND3 { hhll } => {}
            // Instruction::SNP { x, hhll } => {}
            // Instruction::SNG { ad, sr, vt } => {}
            Instruction::JMPI { hhll } => {
                self.program_counter = hhll;
            }
            Instruction::JMC { hhll } => {
                if self.flags.carry {
                    self.program_counter = hhll;
                }
            }
            Instruction::JX { ref c, hhll } => {
                if self.test(c) {
                    self.program_counter = hhll;
                }
            }
            Instruction::JME { y, x, hhll } => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter = hhll;
                }
            }
            Instruction::CALLI { hhll } => {
                self.memory
                    .write_u16(self.stack_pointer as usize, self.program_counter);
                self.stack_pointer += 2;
                self.program_counter = hhll;
            }
            Instruction::RET => {
                self.stack_pointer -= 2;
                self.program_counter = self.memory.read_u16(self.stack_pointer as usize);
            }
            Instruction::JMPR { x } => {
                self.program_counter = self.registers[x as usize];
            }
            Instruction::CX { ref c, hhll } => {
                if self.test(c) {
                    self.memory
                        .write_u16(self.stack_pointer as usize, self.program_counter);
                    self.stack_pointer += 2;
                    self.program_counter = hhll;
                }
            }
            Instruction::CALLR { x } => {
                self.memory
                    .write_u16(self.stack_pointer as usize, self.program_counter);
                self.stack_pointer += 2;
                self.program_counter = self.registers[x as usize];
            }
            Instruction::LDIR { x, hhll } => {
                self.registers[x as usize] = hhll;
            }
            Instruction::LDIS { hhll } => {
                self.stack_pointer = hhll;
            }
            Instruction::LDMI { x, hhll } => {
                self.registers[x as usize] = hhll;
            }
            Instruction::LDMR { y, x } => {
                self.registers[x as usize] =
                    self.memory.read_u16(self.registers[y as usize] as usize);
            }
            Instruction::MOV { y, x } => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            Instruction::STMI { x, hhll } => {
                self.memory
                    .write_u16(hhll as usize, self.registers[x as usize]);
            }
            Instruction::STMR { y, x } => {
                self.memory.write_u16(
                    self.registers[y as usize] as usize,
                    self.registers[x as usize],
                );
            }
            Instruction::ADDI { x, hhll } => {
                self.flags.set_on_add(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_add(hhll);
            }
            Instruction::ADDR2 { y, x } => {
                self.flags
                    .set_on_add(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_add(self.registers[y as usize]);
            }
            Instruction::ADDR3 { y, x, z } => {
                self.flags
                    .set_on_add(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_add(self.registers[y as usize]);
            }
            Instruction::SUBI { x, hhll } => {
                self.flags.set_on_sub(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_sub(hhll);
            }
            Instruction::SUBR2 { y, x } => {
                self.flags
                    .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_sub(self.registers[y as usize]);
            }
            Instruction::SUBR3 { y, x, z } => {
                self.flags
                    .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_sub(self.registers[y as usize]);
            }
            Instruction::CMPI { x, hhll } => {
                self.flags.set_on_sub(self.registers[x as usize], hhll);
            }
            Instruction::CMPR { y, x } => {
                self.flags
                    .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
            }
            Instruction::ANDI { x, hhll } => {
                self.flags.set_on_and(self.registers[x as usize], hhll);
                self.registers[x as usize] = self.registers[x as usize] & hhll;
            }
            Instruction::ANDR2 { y, x } => {
                self.flags
                    .set_on_and(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    self.registers[x as usize] & self.registers[y as usize];
            }
            Instruction::ANDR3 { y, x, z } => {
                self.flags
                    .set_on_and(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    self.registers[x as usize] & self.registers[y as usize];
            }
            Instruction::TSTI { x, hhll } => {
                self.flags.set_on_and(self.registers[x as usize], hhll);
            }
            Instruction::TST { y, x } => {
                self.flags
                    .set_on_and(self.registers[x as usize], self.registers[y as usize]);
            }
            Instruction::ORI { x, hhll } => {
                self.flags.set_on_or(self.registers[x as usize], hhll);
                self.registers[x as usize] = self.registers[x as usize] | hhll;
            }
            Instruction::ORR2 { y, x } => {
                self.flags
                    .set_on_or(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    self.registers[x as usize] | self.registers[y as usize];
            }
            Instruction::ORR3 { y, x, z } => {
                self.flags
                    .set_on_or(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    self.registers[x as usize] | self.registers[y as usize];
            }
            Instruction::XORI { x, hhll } => {
                self.flags.set_on_xor(self.registers[x as usize], hhll);
                self.registers[x as usize] = self.registers[x as usize] ^ hhll;
            }
            Instruction::XORR2 { y, x } => {
                self.flags
                    .set_on_xor(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    self.registers[x as usize] ^ self.registers[y as usize];
            }
            Instruction::XORR3 { y, x, z } => {
                self.flags
                    .set_on_xor(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    self.registers[x as usize] ^ self.registers[y as usize];
            }
            Instruction::MULI { x, hhll } => {
                self.flags.set_on_mul(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_mul(hhll);
            }
            Instruction::MULR2 { y, x } => {
                self.flags
                    .set_on_mul(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_mul(self.registers[y as usize]);
            }
            Instruction::MULR3 { y, x, z } => {
                self.flags
                    .set_on_mul(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_mul(self.registers[y as usize]);
            }
            Instruction::DIVI { x, hhll } => {
                self.flags.set_on_div(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_div(hhll);
            }
            Instruction::DIVR2 { y, x } => {
                self.flags
                    .set_on_div(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_div(self.registers[y as usize]);
            }
            Instruction::DIVR3 { y, x, z } => {
                self.flags
                    .set_on_div(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_div(self.registers[y as usize]);
            }
            // Instruction::MODI { x, hhll } => {
            //     // self.flags.set_on_mod(self.registers[x as usize], hhll);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_mod(hhll);
            // }
            // Instruction::MODR2 { y, x } => {
            //     // self.flags.set_on_mod(self.registers[x as usize], self.registers[y as usize]);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_mod(self.registers[y as usize]);
            // }
            // Instruction::MODR3 { y, x, z } => {
            //     // self.flags.set_on_mod(self.registers[x as usize], self.registers[y as usize]);
            //     // self.registers[z as usize] = (self.registers[x as usize]).wrapping_mod(self.registers[y as usize]);
            // }
            Instruction::REMI { x, hhll } => {
                self.flags.set_on_rem(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_rem(hhll);
            }
            Instruction::REMR2 { y, x } => {
                self.flags
                    .set_on_rem(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_rem(self.registers[y as usize]);
            }
            Instruction::REMR3 { y, x, z } => {
                self.flags
                    .set_on_rem(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_rem(self.registers[y as usize]);
            }
            Instruction::SHLN { x, n } => {
                self.flags.set_on_shl(self.registers[x as usize], n as u16);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(n as u32);
            }
            Instruction::SHRN { x, n } => {
                self.flags.set_on_shr(self.registers[x as usize], n as u16);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_shr(n as u32);
            }
            // Instruction::SARN { x, n } => {
            //     self.flags.set_on_sar(self.registers[x as usize], n as u16);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(n);
            // }
            Instruction::SHLR { y, x } => {
                self.flags
                    .set_on_shl(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_shl(self.registers[y as usize] as u32);
            }
            Instruction::SHRR { y, x } => {
                self.flags
                    .set_on_shr(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_shr(self.registers[y as usize] as u32);
            }
            // Instruction::SARR { y, x } => {
            //     self.flags.set_on_shl(self.registers[x as usize], self.registers[y as usize]);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(self.registers[y as usize]);
            // }
            Instruction::PUSH { x } => {
                self.memory
                    .write_u16(self.stack_pointer as usize, self.registers[x as usize]);
                self.stack_pointer += 2;
            }
            Instruction::POP { x } => {
                self.stack_pointer -= 2;
                self.registers[x as usize] = self.memory.read_u16(self.stack_pointer as usize);
            }
            Instruction::PUSHALL => {
                for r in self.registers.iter() {
                    self.memory.write_u16(self.stack_pointer as usize, *r);
                    self.stack_pointer += 2;
                }
            }
            Instruction::POPALL => {
                for r in self.registers.iter_mut().rev() {
                    *r = self.memory.read_u16(self.stack_pointer as usize);
                    self.stack_pointer -= 2;
                }
            }
            Instruction::PUSHF => {
                self.memory[self.stack_pointer as usize] = From::from(&self.flags);
                self.stack_pointer += 2;
            }
            Instruction::POPF => {
                self.stack_pointer -= 2;
                self.flags = From::from(self.memory[self.stack_pointer as usize]);
            }
            Instruction::PALI { hhll } => {
                for p in self.palette.iter_mut() {
                    p.r = self.memory[self.stack_pointer as usize];
                    p.g = self.memory[(self.stack_pointer + 1) as usize];
                    p.b = self.memory[(self.stack_pointer + 2) as usize];
                    self.stack_pointer += 3;
                }
            }
            Instruction::PALR { x } => {
                let mut i = self.registers[x as usize];
                for p in self.palette.iter_mut() {
                    p.r = self.memory[i as usize];
                    p.g = self.memory[(i + 1) as usize];
                    p.b = self.memory[(i + 2) as usize];
                    i += 3;
                }
            }
            Instruction::NOTI { x, hhll } => {
                self.flags.set_on_not(hhll);
                self.registers[x as usize] = !hhll;
            }
            Instruction::NOTR1 { x } => {
                self.flags.set_on_not(self.registers[x as usize]);
                self.registers[x as usize] = !self.registers[x as usize];
            }
            Instruction::NOTR2 { y, x } => {
                self.flags.set_on_not(self.registers[x as usize]);
                self.registers[x as usize] = !self.registers[y as usize];
            }
            Instruction::NEGI { x, hhll } => {
                self.flags.set_on_neg(hhll);
                self.registers[x as usize] = -(hhll as i16) as u16;
            }
            Instruction::NEGR1 { x } => {
                self.flags.set_on_neg(self.registers[x as usize]);
                self.registers[x as usize] = -(self.registers[x as usize] as i16) as u16;
            }
            Instruction::NEGR2 { y, x } => {
                self.flags.set_on_neg(self.registers[x as usize]);
                self.registers[x as usize] = -(self.registers[y as usize] as i16) as u16;
            }
            _ => {
                println!("Instuction not implemented.");
            }
        };
    }
}
