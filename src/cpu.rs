use flags::Flags;
use instruction::{Condition, Instruction};
use memory::{Memory, Read, Write};

use self::Condition::*;
use self::Instruction::*;

#[derive(Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default)]
pub struct Cpu {
    memory: Memory,
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

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            video_memory: vec![0; 320 * 240],
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        // TODO: Handle errors that fetch could possibly throw.
        let instruction = self.fetch();
        self.program_counter += 4;

        self.execute(&instruction);
    }

    pub fn fetch(&self) -> Instruction {
        let opcode = self.memory.read(self.program_counter);
        Instruction::decode(opcode).unwrap()
    }

    pub fn test(&self, condition: &Condition) -> bool {
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

    pub fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            NOP => self.nop(),
            CLS => self.cls(),
            VBLNK => self.vblnk(),

            // TODO: Translate remaining instructions into new format.
            BGC { n } => {
                self.background = n;
            }
            SPR { width, height } => {
                self.sprite_width = width;
                self.sprite_height = height;
            }
            // DRWI { y, x, hhll } => {}
            // DRWR { y, x, z } => {}
            // RND { x, hhll } => {
            //     self.registers[x as usize] = rand::random::<u16>() % hhll;
            // }
            FLIP {
                flip_horizontal,
                flip_vertical,
            } => {
                self.flip_horizontal = flip_horizontal;
                self.flip_vertical = flip_vertical;
            }
            // SND0 {} => {}
            // SND1 { hhll } => {}
            // SND2 { hhll } => {}
            // SND3 { hhll } => {}
            // SNP { x, hhll } => {}
            // SNG { ad, sr, vt } => {}
            // JMPI { hhll } => {
            //     self.program_counter = hhll;
            // }
            // JMC { hhll } => {
            //     if self.flags.carry {
            //         self.program_counter = hhll;
            //     }
            // }
            // JX { ref c, hhll } => {
            //     if self.test(c) {
            //         self.program_counter = hhll;
            //     }
            // }
            // JME { y, x, hhll } => {
            //     if self.registers[x as usize] == self.registers[y as usize] {
            //         self.program_counter = hhll;
            //     }
            // }
            // CALLI { hhll } => {
            //     self.memory
            //         .write_u16(self.stack_pointer as usize, self.program_counter);
            //     self.stack_pointer += 2;
            //     self.program_counter = hhll;
            // }
            // RET => {
            //     self.stack_pointer -= 2;
            //     self.program_counter = self.memory.read_u16(self.stack_pointer as usize);
            // }
            // JMPR { x } => {
            //     self.program_counter = self.registers[x as usize];
            // }
            // CX { ref c, hhll } => {
            //     if self.test(c) {
            //         self.memory
            //             .write_u16(self.stack_pointer as usize, self.program_counter);
            //         self.stack_pointer += 2;
            //         self.program_counter = hhll;
            //     }
            // }
            // CALLR { x } => {
            //     self.memory
            //         .write_u16(self.stack_pointer as usize, self.program_counter);
            //     self.stack_pointer += 2;
            //     self.program_counter = self.registers[x as usize];
            // }
            // LDIR { x, hhll } => {
            //     self.registers[x as usize] = hhll;
            // }
            // LDIS { hhll } => {
            //     self.stack_pointer = hhll;
            // }
            // LDMI { x, hhll } => {
            //     // NOTE: This instruction is wrong.
            //     self.registers[x as usize] = hhll;
            // }
            // LDMR { y, x } => {
            //     self.registers[x as usize] =
            //         self.memory.read_u16(self.registers[y as usize] as usize);
            // }
            // MOV { y, x } => {
            //     self.registers[x as usize] = self.registers[y as usize];
            // }
            // STMI { x, hhll } => {
            //     self.memory
            //         .write_u16(hhll as usize, self.registers[x as usize]);
            // }
            // STMR { y, x } => {
            //     self.memory.write_u16(
            //         self.registers[y as usize] as usize,
            //         self.registers[x as usize],
            //     );
            // }
            // ADDI { x, hhll } => {
            //     self.flags.set_on_add(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = (self.registers[x as usize]).wrapping_add(hhll);
            // }
            // ADDR2 { y, x } => {
            //     self.flags
            //         .set_on_add(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_add(self.registers[y as usize]);
            // }
            // ADDR3 { y, x, z } => {
            //     self.flags
            //         .set_on_add(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[z as usize] =
            //         (self.registers[x as usize]).wrapping_add(self.registers[y as usize]);
            // }
            // SUBI { x, hhll } => {
            //     self.flags.set_on_sub(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = (self.registers[x as usize]).wrapping_sub(hhll);
            // }
            // SUBR2 { y, x } => {
            //     self.flags
            //         .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_sub(self.registers[y as usize]);
            // }
            // SUBR3 { y, x, z } => {
            //     self.flags
            //         .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[z as usize] =
            //         (self.registers[x as usize]).wrapping_sub(self.registers[y as usize]);
            // }
            // CMPI { x, hhll } => {
            //     self.flags.set_on_sub(self.registers[x as usize], hhll);
            // }
            // CMPR { y, x } => {
            //     self.flags
            //         .set_on_sub(self.registers[x as usize], self.registers[y as usize]);
            // }
            // ANDI { x, hhll } => {
            //     self.flags.set_on_and(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = self.registers[x as usize] & hhll;
            // }
            // ANDR2 { y, x } => {
            //     self.flags
            //         .set_on_and(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         self.registers[x as usize] & self.registers[y as usize];
            // }
            // ANDR3 { y, x, z } => {
            //     self.flags
            //         .set_on_and(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[z as usize] =
            //         self.registers[x as usize] & self.registers[y as usize];
            // }
            // TSTI { x, hhll } => {
            //     self.flags.set_on_and(self.registers[x as usize], hhll);
            // }
            // TSTR { y, x } => {
            //     self.flags
            //         .set_on_and(self.registers[x as usize], self.registers[y as usize]);
            // }
            // ORI { x, hhll } => {
            //     self.flags.set_on_or(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = self.registers[x as usize] | hhll;
            // }
            // ORR2 { y, x } => {
            //     self.flags
            //         .set_on_or(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         self.registers[x as usize] | self.registers[y as usize];
            // }
            // ORR3 { y, x, z } => {
            //     self.flags
            //         .set_on_or(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[z as usize] =
            //         self.registers[x as usize] | self.registers[y as usize];
            // }
            // XORI { x, hhll } => {
            //     self.flags.set_on_xor(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = self.registers[x as usize] ^ hhll;
            // }
            // XORR2 { y, x } => {
            //     self.flags
            //         .set_on_xor(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         self.registers[x as usize] ^ self.registers[y as usize];
            // }
            // XORR3 { y, x, z } => {
            //     self.flags
            //         .set_on_xor(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[z as usize] =
            //         self.registers[x as usize] ^ self.registers[y as usize];
            // }
            // MULI { x, hhll } => {
            //     self.flags.set_on_mul(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = (self.registers[x as usize]).wrapping_mul(hhll);
            // }
            // MULR2 { y, x } => {
            //     self.flags
            //         .set_on_mul(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_mul(self.registers[y as usize]);
            // }
            // MULR3 { y, x, z } => {
            //     self.flags
            //         .set_on_mul(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[z as usize] =
            //         (self.registers[x as usize]).wrapping_mul(self.registers[y as usize]);
            // }
            // DIVI { x, hhll } => {
            //     self.flags.set_on_div(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = (self.registers[x as usize]).wrapping_div(hhll);
            // }
            // DIVR2 { y, x } => {
            //     self.flags
            //         .set_on_div(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_div(self.registers[y as usize]);
            // }
            // DIVR3 { y, x, z } => {
            //     self.flags
            //         .set_on_div(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_div(self.registers[y as usize]);
            // }
            // // MODI { x, hhll } => {
            // //     // self.flags.set_on_mod(self.registers[x as usize], hhll);
            // //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_mod(hhll);
            // // }
            // // MODR2 { y, x } => {
            // //     // self.flags.set_on_mod(self.registers[x as usize], self.registers[y as usize]);
            // //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_mod(self.registers[y as usize]);
            // // }
            // // MODR3 { y, x, z } => {
            // //     // self.flags.set_on_mod(self.registers[x as usize], self.registers[y as usize]);
            // //     // self.registers[z as usize] = (self.registers[x as usize]).wrapping_mod(self.registers[y as usize]);
            // // }
            // REMI { x, hhll } => {
            //     self.flags.set_on_rem(self.registers[x as usize], hhll);
            //     self.registers[x as usize] = (self.registers[x as usize]).wrapping_rem(hhll);
            // }
            // REMR2 { y, x } => {
            //     self.flags
            //         .set_on_rem(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_rem(self.registers[y as usize]);
            // }
            // REMR3 { y, x, z } => {
            //     self.flags
            //         .set_on_rem(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[z as usize] =
            //         (self.registers[x as usize]).wrapping_rem(self.registers[y as usize]);
            // }
            // SHLN { x, n } => {
            //     self.flags.set_on_shl(self.registers[x as usize], n as u16);
            //     self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(n as u32);
            // }
            // SHRN { x, n } => {
            //     self.flags.set_on_shr(self.registers[x as usize], n as u16);
            //     self.registers[x as usize] = (self.registers[x as usize]).wrapping_shr(n as u32);
            // }
            // // SARN { x, n } => {
            // //     self.flags.set_on_sar(self.registers[x as usize], n as u16);
            // //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(n);
            // // }
            // SHLR { y, x } => {
            //     self.flags
            //         .set_on_shl(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_shl(self.registers[y as usize] as u32);
            // }
            // SHRR { y, x } => {
            //     self.flags
            //         .set_on_shr(self.registers[x as usize], self.registers[y as usize]);
            //     self.registers[x as usize] =
            //         (self.registers[x as usize]).wrapping_shr(self.registers[y as usize] as u32);
            // }
            // // SARR { y, x } => {
            // //     self.flags.set_on_shl(self.registers[x as usize], self.registers[y as usize]);
            // //     // self.registers[x as usize] = (self.registers[x as usize]).wrapping_shl(self.registers[y as usize]);
            // // }
            // PUSH { x } => {
            //     self.memory
            //         .write_u16(self.stack_pointer as usize, self.registers[x as usize]);
            //     self.stack_pointer += 2;
            // }
            // POP { x } => {
            //     self.stack_pointer -= 2;
            //     self.registers[x as usize] = self.memory.read_u16(self.stack_pointer as usize);
            // }
            // PUSHALL => for r in self.registers.iter() {
            //     self.memory.write_u16(self.stack_pointer as usize, *r);
            //     self.stack_pointer += 2;
            // },
            // POPALL => for r in self.registers.iter_mut().rev() {
            //     *r = self.memory.read_u16(self.stack_pointer as usize);
            //     self.stack_pointer -= 2;
            // },
            // PUSHF => {
            //     self.memory[self.stack_pointer as usize] = From::from(&self.flags);
            //     self.stack_pointer += 2;
            // }
            // POPF => {
            //     self.stack_pointer -= 2;
            //     self.flags = From::from(self.memory[self.stack_pointer as usize]);
            // }
            // PALI { hhll } => for p in self.palette.iter_mut() {
            //     p.r = self.memory[self.stack_pointer as usize];
            //     p.g = self.memory[(self.stack_pointer + 1) as usize];
            //     p.b = self.memory[(self.stack_pointer + 2) as usize];
            //     self.stack_pointer += 3;
            // },
            // PALR { x } => {
            //     let mut i = self.registers[x as usize];
            //     for p in self.palette.iter_mut() {
            //         p.r = self.memory[i as usize];
            //         p.g = self.memory[(i + 1) as usize];
            //         p.b = self.memory[(i + 2) as usize];
            //         i += 3;
            //     }
            // }
            // NOTI { x, hhll } => {
            //     self.flags.set_on_not(hhll);
            //     self.registers[x as usize] = !hhll;
            // }
            // NOTR1 { x } => {
            //     self.flags.set_on_not(self.registers[x as usize]);
            //     self.registers[x as usize] = !self.registers[x as usize];
            // }
            // NOTR2 { y, x } => {
            //     self.flags.set_on_not(self.registers[x as usize]);
            //     self.registers[x as usize] = !self.registers[y as usize];
            // }
            // NEGI { x, hhll } => {
            //     self.flags.set_on_neg(hhll);
            //     self.registers[x as usize] = -(hhll as i16) as u16;
            // }
            // NEGR1 { x } => {
            //     self.flags.set_on_neg(self.registers[x as usize]);
            //     self.registers[x as usize] = -(self.registers[x as usize] as i16) as u16;
            // }
            // NEGR2 { y, x } => {
            //     self.flags.set_on_neg(self.registers[x as usize]);
            //     self.registers[x as usize] = -(self.registers[y as usize] as i16) as u16;
            // }
            _ => {
                println!("Instuction not implemented.");
            }
        };
    }

    fn nop(&mut self) {}
    fn cls(&mut self) {
        self.background = 0;
        self.video_memory.clear();
    }
    fn vblnk(&mut self) {}
    fn bgc(&mut self, n: u8) {}
    fn spr(&mut self, width: u8, height: u8) {}
    fn drwi(&mut self, x: u8, y: u8, address: u16) {}
    fn drwr(&mut self, x: u8, y: u8, z: u8) {}
    fn rnd(&mut self, x: u8, address: u16) {}
    fn flip(&mut self, flip_horizontal: bool, flip_vertical: bool) {}
    fn snd0(&mut self) {}
    fn snd1(&mut self, address: u16) {}
    fn snd2(&mut self, address: u16) {}
    fn snd3(&mut self, address: u16) {}
    fn snp(&mut self, x: u8, address: u16) {}
    fn sng(&mut self, attack: u8, decay: u8, sustain: u8, release: u8, volume: u8, wave: u8) {}
    fn jmpi(&mut self, address: u16) {}
    fn jmc(&mut self, address: u16) {}
    fn jx(&mut self, condition: Condition, address: u16) {}
    fn jme(&mut self, x: u8, y: u8, address: u16) {}
    fn calli(&mut self, address: u16) {}
    fn ret(&mut self) {}
    fn jmpr(&mut self, x: u8) {}
    fn cx(&mut self, condition: Condition, address: u16) {}
    fn callr(&mut self, x: u8) {}
    fn ldir(&mut self, x: u8, address: u16) {}
    fn ldis(&mut self, address: u16) {}
    fn ldmi(&mut self, x: u8, address: u16) {}
    fn ldmr(&mut self, x: u8, y: u8) {}
    fn mov(&mut self, x: u8, y: u8) {}
    fn stmi(&mut self, x: u8, address: u16) {}
    fn stmr(&mut self, x: u8, y: u8) {}
    fn addi(&mut self, x: u8, address: u16) {}
    fn addr2(&mut self, x: u8, y: u8) {}
    fn addr3(&mut self, x: u8, y: u8, z: u8) {}
    fn subi(&mut self, x: u8, address: u16) {}
    fn subr2(&mut self, x: u8, y: u8) {}
    fn subr3(&mut self, x: u8, y: u8, z: u8) {}
    fn cmpi(&mut self, x: u8, address: u16) {}
    fn cmpr(&mut self, x: u8, y: u8) {}
    fn andi(&mut self, x: u8, address: u16) {}
    fn andr2(&mut self, x: u8, y: u8) {}
    fn andr3(&mut self, x: u8, y: u8, z: u8) {}
    fn tsti(&mut self, x: u8, address: u16) {}
    fn tstr(&mut self, x: u8, y: u8) {}
    fn ori(&mut self, x: u8, address: u16) {}
    fn orr2(&mut self, x: u8, y: u8) {}
    fn orr3(&mut self, x: u8, y: u8, z: u8) {}
    fn xori(&mut self, x: u8, address: u16) {}
    fn xorr2(&mut self, x: u8, y: u8) {}
    fn xorr3(&mut self, x: u8, y: u8, z: u8) {}
    fn muli(&mut self, x: u8, address: u16) {}
    fn mulr2(&mut self, x: u8, y: u8) {}
    fn mulr3(&mut self, x: u8, y: u8, z: u8) {}
    fn divi(&mut self, x: u8, address: u16) {}
    fn divr2(&mut self, x: u8, y: u8) {}
    fn divr3(&mut self, x: u8, y: u8, z: u8) {}
    fn modi(&mut self, x: u8, address: u16) {}
    fn modr2(&mut self, x: u8, y: u8) {}
    fn modr3(&mut self, x: u8, y: u8, z: u8) {}
    fn remi(&mut self, x: u8, address: u16) {}
    fn remr2(&mut self, x: u8, y: u8) {}
    fn remr3(&mut self, x: u8, y: u8, z: u8) {}
    fn shln(&mut self, x: u8, n: u8) {}
    fn shrn(&mut self, x: u8, n: u8) {}
    fn sarn(&mut self, x: u8, n: u8) {}
    fn shlr(&mut self, x: u8, y: u8) {}
    fn shrr(&mut self, x: u8, y: u8) {}
    fn sarr(&mut self, x: u8, y: u8) {}
    fn push(&mut self, x: u8) {}
    fn pop(&mut self, x: u8) {}
    fn pushall(&mut self) {}
    fn popall(&mut self) {}
    fn pushf(&mut self) {}
    fn popf(&mut self) {}
    fn pali(&mut self, address: u16) {}
    fn palr(&mut self, x: u8) {}
    fn noti(&mut self, x: u8, address: u16) {}
    fn notr1(&mut self, x: u8) {}
    fn notr2(&mut self, x: u8, y: u8) {}
    fn negi(&mut self, x: u8, address: u16) {}
    fn negr1(&mut self, x: u8) {}
    fn negr2(&mut self, x: u8, y: u8) {}
}
