use condition::{Condition, Condition::*};
use flags::Flags;
use instruction::{Instruction, Instruction::*};

#[derive(Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default)]
pub struct Cpu {
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
    fn read_u32(&self, index: usize) -> u32;
    fn write_u32(&mut self, index: usize, value: u32);
}

impl Memory for Vec<u8> {
    fn read_u16(&self, index: usize) -> u16 {
        ((self[index] as u16) << 8) & (self[index + 1] as u16)
    }

    fn write_u16(&mut self, index: usize, value: u16) {
        self[index + 0] = (value & 0xFF00 >> 8) as u8;
        self[index + 1] = (value & 0x00FF) as u8;
    }

    fn read_u32(&self, index: usize) -> u32 {
        ((self[index] as u32) << 24) & ((self[index + 1] as u32) << 16)
            & ((self[index + 2] as u32) << 8) & (self[index + 4] as u32)
    }

    fn write_u32(&mut self, index: usize, value: u32) {
        self[index + 0] = (value & 0xFF000000 >> 24) as u8;
        self[index + 1] = (value & 0x00FF0000 >> 16) as u8;
        self[index + 2] = (value & 0x0000FF00 >> 8) as u8;
        self[index + 3] = (value & 0x000000FF) as u8;
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: vec![0; 65_536],
            video_memory: vec![0; 320 * 240],
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.program_counter += 4;

        self.execute(&instruction);
    }

    fn fetch(&self) -> Instruction {
        let opcode = self.memory.read_u32(self.program_counter as usize);
        Instruction::decode(opcode).unwrap()
    }

    fn test(&self, condition: &Condition) -> bool {
        match *condition {
            Z => self.flags.zero,
            NZ => !self.flags.zero,
            N => self.flags.negative,
            NN => !self.flags.negative,
            P => !self.flags.negative && !self.flags.zero,
            O => self.flags.overflow,
            NO => !self.flags.overflow,
            A => !self.flags.carry && !self.flags.zero,
            AE => !self.flags.carry,
            B => self.flags.carry,
            BE => self.flags.carry || self.flags.zero,
            G => self.flags.overflow == self.flags.negative && !self.flags.zero,
            GE => self.flags.overflow == self.flags.negative,
            L => self.flags.overflow != self.flags.negative,
            LE => self.flags.overflow != self.flags.negative || self.flags.zero,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            NOP => {}
            CLS => {
                self.background = 0u8;
                self.video_memory.clear();
            }
            // VBLNK => {}
            BGC { n } => {
                self.background = n;
            }
            SPR { ll, hh } => {
                self.sprite_width = ll;
                self.sprite_height = hh;
            }
            // DRWI { y, x, hhll } => {}
            // DRWR { y, x, z } => {}
            // RND { x, hhll } => {
            //     self.registers[x as usize] = rand::random::<u16>() % hhll;
            // }
            FLIP { fh, fv } => {
                self.flip_horizontal = fh;
                self.flip_vertical = fv;
            }
            // SND0 {} => {}
            // SND1 { hhll } => {}
            // SND2 { hhll } => {}
            // SND3 { hhll } => {}
            // SNP { x, hhll } => {}
            // SNG { ad, sr, vt } => {}
            JMPI { hhll } => {
                self.program_counter = hhll;
            }
            JMC { hhll } => {
                if self.flags.carry {
                    self.program_counter = hhll;
                }
            }
            JX { ref c, hhll } => {
                if self.test(c) {
                    self.program_counter = hhll;
                }
            }
            JME { y, x, hhll } => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter = hhll;
                }
            }
            CALLI { hhll } => {
                self.memory
                    .write_u16(self.stack_pointer as usize, self.program_counter);
                self.stack_pointer += 2;
                self.program_counter = hhll;
            }
            RET => {
                self.stack_pointer -= 2;
                self.program_counter = self.memory.read_u16(self.stack_pointer as usize);
            }
            JMPR { x } => {
                self.program_counter = self.registers[x as usize];
            }
            CX { ref c, hhll } => {
                if self.test(c) {
                    self.memory
                        .write_u16(self.stack_pointer as usize, self.program_counter);
                    self.stack_pointer += 2;
                    self.program_counter = hhll;
                }
            }
            CALLR { x } => {
                self.memory
                    .write_u16(self.stack_pointer as usize, self.program_counter);
                self.stack_pointer += 2;
                self.program_counter = self.registers[x as usize];
            }
            LDIR { x, hhll } => {
                self.registers[x as usize] = hhll;
            }
            LDIS { hhll } => {
                self.stack_pointer = hhll;
            }
            LDMI { x, hhll } => {
                self.registers[x as usize] = hhll;
            }
            LDMR { y, x } => {
                self.registers[x as usize] =
                    self.memory.read_u16(self.registers[y as usize] as usize);
            }
            MOV { y, x } => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            STMI { x, hhll } => {
                self.memory
                    .write_u16(hhll as usize, self.registers[x as usize]);
            }
            STMR { y, x } => {
                self.memory.write_u16(
                    self.registers[y as usize] as usize,
                    self.registers[x as usize],
                );
            }
            ADDI { x, hhll } => {
                self.flags.set_on_add(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_add(hhll);
            }
            ADDR2 { y, x } => {
                self.flags
                    .set_on_add(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_add(self.registers[y as usize]);
            }
            ADDR3 { y, x, z } => {
                self.flags
                    .set_on_add(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_add(self.registers[y as usize]);
            }
            SUBI { x, hhll } => {
                self.flags.set_on_sub(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_sub(hhll);
            }
            SUBR2 { y, x } => {
                self.flags
                    .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_sub(self.registers[y as usize]);
            }
            SUBR3 { y, x, z } => {
                self.flags
                    .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_sub(self.registers[y as usize]);
            }
            CMPI { x, hhll } => {
                self.flags.set_on_sub(self.registers[x as usize], hhll);
            }
            CMPR { y, x } => {
                self.flags
                    .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
            }
            ANDI { x, hhll } => {
                self.flags.set_on_and(self.registers[x as usize], hhll);
                self.registers[x as usize] = self.registers[x as usize] & hhll;
            }
            ANDR2 { y, x } => {
                self.flags
                    .set_on_and(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    self.registers[x as usize] & self.registers[y as usize];
            }
            ANDR3 { y, x, z } => {
                self.flags
                    .set_on_and(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    self.registers[x as usize] & self.registers[y as usize];
            }
            TSTI { x, hhll } => {
                self.flags.set_on_and(self.registers[x as usize], hhll);
            }
            TST { y, x } => {
                self.flags
                    .set_on_and(self.registers[x as usize], self.registers[y as usize]);
            }
            ORI { x, hhll } => {
                self.flags.set_on_or(self.registers[x as usize], hhll);
                self.registers[x as usize] = self.registers[x as usize] | hhll;
            }
            ORR2 { y, x } => {
                self.flags
                    .set_on_or(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    self.registers[x as usize] | self.registers[y as usize];
            }
            ORR3 { y, x, z } => {
                self.flags
                    .set_on_or(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    self.registers[x as usize] | self.registers[y as usize];
            }
            XORI { x, hhll } => {
                self.flags.set_on_xor(self.registers[x as usize], hhll);
                self.registers[x as usize] = self.registers[x as usize] ^ hhll;
            }
            XORR2 { y, x } => {
                self.flags
                    .set_on_xor(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    self.registers[x as usize] ^ self.registers[y as usize];
            }
            XORR3 { y, x, z } => {
                self.flags
                    .set_on_xor(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    self.registers[x as usize] ^ self.registers[y as usize];
            }
            MULI { x, hhll } => {
                self.flags.set_on_mul(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_mul(hhll);
            }
            MULR2 { y, x } => {
                self.flags
                    .set_on_mul(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_mul(self.registers[y as usize]);
            }
            MULR3 { y, x, z } => {
                self.flags
                    .set_on_mul(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_mul(self.registers[y as usize]);
            }
            DIVI { x, hhll } => {
                self.flags.set_on_div(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_div(hhll);
            }
            DIVR2 { y, x } => {
                self.flags
                    .set_on_div(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_div(self.registers[y as usize]);
            }
            DIVR3 { y, x, z } => {
                self.flags
                    .set_on_div(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_div(self.registers[y as usize]);
            }
            // MODI { x, hhll } => {
            //     // self.flags.set_on_mod(self.registers[x as usize], hhll);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_mod(hhll);
            // }
            // MODR2 { y, x } => {
            //     // self.flags.set_on_mod(self.registers[x as usize], self.registers[y as usize]);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_mod(self.registers[y as usize]);
            // }
            // MODR3 { y, x, z } => {
            //     // self.flags.set_on_mod(self.registers[x as usize], self.registers[y as usize]);
            //     // self.registers[z as usize] = (self.registers[x as usize]).wrapping_mod(self.registers[y as usize]);
            // }
            REMI { x, hhll } => {
                self.flags.set_on_rem(self.registers[x as usize], hhll);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_rem(hhll);
            }
            REMR2 { y, x } => {
                self.flags
                    .set_on_rem(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_rem(self.registers[y as usize]);
            }
            REMR3 { y, x, z } => {
                self.flags
                    .set_on_rem(self.registers[x as usize], self.registers[y as usize]);
                self.registers[z as usize] =
                    (self.registers[x as usize]).wrapping_rem(self.registers[y as usize]);
            }
            SHLN { x, n } => {
                self.flags.set_on_shl(self.registers[x as usize], n as u16);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(n as u32);
            }
            SHRN { x, n } => {
                self.flags.set_on_shr(self.registers[x as usize], n as u16);
                self.registers[x as usize] = (self.registers[x as usize]).wrapping_shr(n as u32);
            }
            // SARN { x, n } => {
            //     self.flags.set_on_sar(self.registers[x as usize], n as u16);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(n);
            // }
            SHLR { y, x } => {
                self.flags
                    .set_on_shl(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_shl(self.registers[y as usize] as u32);
            }
            SHRR { y, x } => {
                self.flags
                    .set_on_shr(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] =
                    (self.registers[x as usize]).wrapping_shr(self.registers[y as usize] as u32);
            }
            // SARR { y, x } => {
            //     self.flags.set_on_shl(self.registers[x as usize], self.registers[y as usize]);
            //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(self.registers[y as usize]);
            // }
            PUSH { x } => {
                self.memory
                    .write_u16(self.stack_pointer as usize, self.registers[x as usize]);
                self.stack_pointer += 2;
            }
            POP { x } => {
                self.stack_pointer -= 2;
                self.registers[x as usize] = self.memory.read_u16(self.stack_pointer as usize);
            }
            PUSHALL => for r in self.registers.iter() {
                self.memory.write_u16(self.stack_pointer as usize, *r);
                self.stack_pointer += 2;
            },
            POPALL => for r in self.registers.iter_mut().rev() {
                *r = self.memory.read_u16(self.stack_pointer as usize);
                self.stack_pointer -= 2;
            },
            PUSHF => {
                self.memory[self.stack_pointer as usize] = From::from(&self.flags);
                self.stack_pointer += 2;
            }
            POPF => {
                self.stack_pointer -= 2;
                self.flags = From::from(self.memory[self.stack_pointer as usize]);
            }
            PALI { hhll } => for p in self.palette.iter_mut() {
                p.r = self.memory[self.stack_pointer as usize];
                p.g = self.memory[(self.stack_pointer + 1) as usize];
                p.b = self.memory[(self.stack_pointer + 2) as usize];
                self.stack_pointer += 3;
            },
            PALR { x } => {
                let mut i = self.registers[x as usize];
                for p in self.palette.iter_mut() {
                    p.r = self.memory[i as usize];
                    p.g = self.memory[(i + 1) as usize];
                    p.b = self.memory[(i + 2) as usize];
                    i += 3;
                }
            }
            NOTI { x, hhll } => {
                self.flags.set_on_not(hhll);
                self.registers[x as usize] = !hhll;
            }
            NOTR1 { x } => {
                self.flags.set_on_not(self.registers[x as usize]);
                self.registers[x as usize] = !self.registers[x as usize];
            }
            NOTR2 { y, x } => {
                self.flags.set_on_not(self.registers[x as usize]);
                self.registers[x as usize] = !self.registers[y as usize];
            }
            NEGI { x, hhll } => {
                self.flags.set_on_neg(hhll);
                self.registers[x as usize] = -(hhll as i16) as u16;
            }
            NEGR1 { x } => {
                self.flags.set_on_neg(self.registers[x as usize]);
                self.registers[x as usize] = -(self.registers[x as usize] as i16) as u16;
            }
            NEGR2 { y, x } => {
                self.flags.set_on_neg(self.registers[x as usize]);
                self.registers[x as usize] = -(self.registers[y as usize] as i16) as u16;
            }
            _ => {
                println!("Instuction not implemented.");
            }
        };
    }
}
