use flags::Flags;
use instruction::{Condition, Instruction};
use memory::{Memory, Read, Write};
use registers::Registers;

use self::Condition::*;
use self::Instruction::*;

#[derive(Default)]
pub struct Cpu {
    memory: Memory,
    registers: Registers,
    flags: Flags,
    program_counter: u16,
    stack_pointer: u16,
    video_memory: Vec<u8>, // TODO: Generalise Memory to support video_memory.
    palette: [(u8, u8, u8); 16],
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

        self.execute(instruction);
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

    pub fn execute(&mut self, instruction: Instruction) {
        // NOTE: Currently this function consumes the instruction.
        // This may not be what we want... But the other ways are ugly...
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match instruction {
            NOP => self.nop(),
            CLS => self.cls(),
            VBLNK => self.vblnk(),
            BGC { n } => self.bgc(n),
            SPR { width, height } => self.spr(width, height),
            DRWI { x, y, address } => self.drwi(x, y, address),
            DRWR { x, y, z } => self.drwr(x, y, z),
            RND { x, address } => self.rnd(x, address),
            FLIP { flip_horizontal, flip_vertical } => self.flip(flip_horizontal, flip_vertical),
            SND0 => self.snd0(),
            SND1 { address } => self.snd1(address),
            SND2 { address } => self.snd2(address),
            SND3 { address } => self.snd3(address),
            SNP { x, address } => self.snp(x, address),
            SNG { attack, decay, sustain, release, volume, wave } => self.sng(attack, decay, sustain, release, volume, wave),
            JMPI { address } => self.jmpi(address),
            JMC { address } => self.jmc(address),
            JX { condition, address } => self.jx(&condition, address),
            JME { x, y, address } => self.jme(x, y, address),
            CALLI { address } => self.calli(address),
            RET => self.ret(),
            JMPR { x } => self.jmpr(x),
            CX { condition, address } => self.cx(&condition, address),
            CALLR { x } => self.callr(x),
            LDIR { x, immediate } => self.ldir(x, immediate),
            LDIS { immediate } => self.ldis(immediate),
            LDMI { x, address } => self.ldmi(x, address),
            LDMR { x, y } => self.ldmr(x, y),
            MOV { x, y } => self.mov(x, y),
            STMI { x, address } => self.stmi(x, address),
            STMR { x, y } => self.stmr(x, y),
            ADDI { x, immediate } => self.addi(x, immediate),
            ADDR2 { x, y } => self.addr2(x, y),
            ADDR3 { x, y, z } => self.addr3(x, y, z),
            SUBI { x, immediate } => self.subi(x, immediate),
            SUBR2 { x, y } => self.subr2(x, y),
            SUBR3 { x, y, z } => self.subr3(x, y, z),
            CMPI { x, immediate } => self.cmpi(x, immediate),
            CMPR { x, y } => self.cmpr(x, y),
            ANDI { x, immediate } => self.andi(x, immediate),
            ANDR2 { x, y } => self.andr2(x, y),
            ANDR3 { x, y, z } => self.andr3(x, y, z),
            TSTI { x, immediate } => self.tsti(x, immediate),
            TSTR { x, y } => self.tstr(x, y),
            ORI { x, immediate } => self.ori(x, immediate),
            ORR2 { x, y } => self.orr2(x, y),
            ORR3 { x, y, z } => self.orr3(x, y, z),
            XORI { x, immediate } => self.xori(x, immediate),
            XORR2 { x, y } => self.xorr2(x, y),
            XORR3 { x, y, z } => self.xorr3(x, y, z),
            MULI { x, immediate } => self.muli(x, immediate),
            MULR2 { x, y } => self.mulr2(x, y),
            MULR3 { x, y, z } => self.mulr3(x, y, z),
            DIVI { x, immediate } => self.divi(x, immediate),
            DIVR2 { x, y } => self.divr2(x, y),
            DIVR3 { x, y, z } => self.divr3(x, y, z),
            MODI { x, immediate } => self.modi(x, immediate),
            MODR2 { x, y } => self.modr2(x, y),
            MODR3 { x, y, z } => self.modr3(x, y, z),
            REMI { x, immediate } => self.remi(x, immediate),
            REMR2 { x, y } => self.remr2(x, y),
            REMR3 { x, y, z } => self.remr3(x, y, z),
            SHLN { x, n } => self.shln(x, n),
            SHRN { x, n } => self.shrn(x, n),
            SARN { x, n } => self.sarn(x, n),
            SHLR { x, y } => self.shlr(x, y),
            SHRR { x, y } => self.shrr(x, y),
            SARR { x, y } => self.sarr(x, y),
            PUSH { x } => self.push(x),
            POP { x } => self.pop(x),
            PUSHALL => self.pushall(),
            POPALL => self.popall(),
            PUSHF => self.pushf(),
            POPF => self.popf(),
            PALI { address } => self.pali(address),
            PALR { x } => self.palr(x),
            NOTI { x, immediate } => self.noti(x, immediate),
            NOTR1 { x } => self.notr1(x),
            NOTR2 { x, y } => self.notr2(x, y),
            NEGI { x, immediate } => self.negi(x, immediate),
            NEGR1 { x } => self.negr1(x),
            NEGR2 { x, y } => self.negr2(x, y),
        };
    }

    fn nop(&mut self) {}

    fn cls(&mut self) {
        self.background = 0;
        self.video_memory.clear();
    }

    fn vblnk(&mut self) {
        // TODO
    }

    fn bgc(&mut self, n: u8) {
        self.background = n;
    }

    fn spr(&mut self, width: u8, height: u8) {
        self.sprite_width = width;
        self.sprite_height = height;
    }

    fn drwi(&mut self, x: u8, y: u8, address: u16) {
        // TODO
    }

    fn drwr(&mut self, x: u8, y: u8, z: u8) {
        // TODO
    }

    fn rnd(&mut self, x: u8, address: u16) {
        // TODO
    }

    fn flip(&mut self, flip_horizontal: bool, flip_vertical: bool) {
        self.flip_horizontal = flip_horizontal;
        self.flip_vertical = flip_vertical;
    }

    fn snd0(&mut self) {
        // TODO
    }

    fn snd1(&mut self, address: u16) {
        // TODO
    }

    fn snd2(&mut self, address: u16) {
        // TODO
    }

    fn snd3(&mut self, address: u16) {
        // TODO
    }

    fn snp(&mut self, x: u8, address: u16) {
        // TODO
    }

    fn sng(&mut self, attack: u8, decay: u8, sustain: u8, release: u8, volume: u8, wave: u8) {
        // TODO
    }

    fn jmpi(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn jmc(&mut self, address: u16) {
        if self.flags.carry {
            self.program_counter = address;
        }
    }

    fn jx(&mut self, condition: &Condition, address: u16) {
        if self.test(condition) {
            self.program_counter = address;
        }
    }

    fn jme(&mut self, x: u8, y: u8, address: u16) {
        if self.registers[x] == self.registers[y] {
            self.program_counter = address;
        }
    }

    fn calli(&mut self, address: u16) {
        self.memory.write(self.stack_pointer, self.program_counter);
        self.stack_pointer += 2;
        self.program_counter = address;
    }

    fn ret(&mut self) {
        self.stack_pointer -= 2;
        self.program_counter = self.memory.read(self.stack_pointer);
    }

    fn jmpr(&mut self, x: u8) {
        self.program_counter = self.registers[x];
    }

    fn cx(&mut self, condition: &Condition, address: u16) {
        if self.test(condition) {
            self.memory.write(self.stack_pointer, self.program_counter);
            self.stack_pointer += 2;
            self.program_counter = address;
        }
    }

    fn callr(&mut self, x: u8) {
        self.memory.write(self.stack_pointer, self.program_counter);
        self.stack_pointer += 2;
        self.program_counter = self.registers[x];
    }

    fn ldir(&mut self, x: u8, immediate: u16) {
        self.registers[x] = immediate;
    }

    fn ldis(&mut self, immediate: u16) {
        self.stack_pointer = immediate;
    }

    fn ldmi(&mut self, x: u8, address: u16) {
        self.registers[x] = self.memory.read(address);
    }

    fn ldmr(&mut self, x: u8, y: u8) {
        self.registers[x] = self.memory.read(self.registers[y]);
    }

    fn mov(&mut self, x: u8, y: u8) {
        self.registers[x] = self.registers[y];
    }

    fn stmi(&mut self, x: u8, address: u16) {
        self.memory.write(address, self.registers[x]);
    }

    fn stmr(&mut self, x: u8, y: u8) {
        self.memory.write(self.registers[y], self.registers[x]);
    }

    fn addi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_add(self.registers[x], immediate);
        self.registers[x] = u16::wrapping_add(self.registers[x], immediate);
    }

    fn addr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_add(self.registers[x], self.registers[y]);
        self.registers[x] = u16::wrapping_add(self.registers[x], self.registers[y]);
    }

    fn addr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_add(self.registers[x], self.registers[y]);
        self.registers[z] = u16::wrapping_add(self.registers[x], self.registers[y]);
    }

    fn subi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_sub(self.registers[x], immediate);
        self.registers[x] = u16::wrapping_sub(self.registers[x], immediate);
    }

    fn subr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_sub(self.registers[x], self.registers[y]);
        self.registers[x] = u16::wrapping_sub(self.registers[x], self.registers[y]);
    }

    fn subr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_sub(self.registers[x], self.registers[y]);
        self.registers[z] = u16::wrapping_sub(self.registers[x], self.registers[y]);
    }

    fn cmpi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_sub(self.registers[x], immediate);
    }

    fn cmpr(&mut self, x: u8, y: u8) {
        self.flags.set_on_sub(self.registers[x], self.registers[y]);
    }

    fn andi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_and(self.registers[x], immediate);
        self.registers[x] = self.registers[x] & immediate;
    }

    fn andr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_and(self.registers[x], self.registers[y]);
        self.registers[x] = self.registers[x] & self.registers[y];
    }

    fn andr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_and(self.registers[x], self.registers[y]);
        self.registers[z] = self.registers[x] & self.registers[y];
    }

    fn tsti(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_and(self.registers[x], immediate);
    }

    fn tstr(&mut self, x: u8, y: u8) {
        self.flags.set_on_and(self.registers[x], self.registers[y]);
    }

    fn ori(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_or(self.registers[x], immediate);
        self.registers[x] = self.registers[x] | immediate;
    }

    fn orr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_or(self.registers[x], self.registers[y]);
        self.registers[x] = self.registers[x] | self.registers[y];
    }

    fn orr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_or(self.registers[x], self.registers[y]);
        self.registers[z] = self.registers[x] | self.registers[y];
    }

    fn xori(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_xor(self.registers[x], immediate);
        self.registers[x] = self.registers[x] ^ immediate;
    }

    fn xorr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_xor(self.registers[x], self.registers[y]);
        self.registers[x] = self.registers[x] ^ self.registers[y];
    }

    fn xorr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_xor(self.registers[x], self.registers[y]);
        self.registers[z] = self.registers[x] ^ self.registers[y];
    }

    fn muli(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_mul(self.registers[x], immediate);
        self.registers[x] = u16::wrapping_mul(self.registers[x], immediate);
    }

    fn mulr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_mul(self.registers[x], self.registers[y]);
        self.registers[x] = u16::wrapping_mul(self.registers[x], self.registers[y]);
    }

    fn mulr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_mul(self.registers[x], self.registers[y]);
        self.registers[z] = u16::wrapping_mul(self.registers[x], self.registers[y]);
    }

    fn divi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_div(self.registers[x], immediate);
        self.registers[x] = u16::wrapping_div(self.registers[x], immediate);
    }

    fn divr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_div(self.registers[x], self.registers[y]);
        self.registers[x] = u16::wrapping_div(self.registers[x], self.registers[y]);
    }

    fn divr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_div(self.registers[x], self.registers[y]);
        self.registers[z] = u16::wrapping_div(self.registers[x], self.registers[y]);
    }

    // TODO: What is the difference between mod and rem?
    fn modi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_mod(self.registers[x], immediate);
        // self.registers[x] = u16::wrapping_mod(self.registers[x], immediate);
    }

    fn modr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_mod(self.registers[x], self.registers[y]);
        // self.registers[x] = u16::wrapping_mod(self.registers[x], self.registers[y]);
    }

    fn modr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_mod(self.registers[x], self.registers[y]);
        // self.registers[z] = u16::wrapping_mod(self.registers[x], self.registers[y]);
    }

    fn remi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_rem(self.registers[x], immediate);
        self.registers[x] = u16::wrapping_rem(self.registers[x], immediate);
    }

    fn remr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_rem(self.registers[x], self.registers[y]);
        self.registers[x] = u16::wrapping_rem(self.registers[x], self.registers[y]);
    }

    fn remr3(&mut self, x: u8, y: u8, z: u8) {
        self.flags.set_on_rem(self.registers[x], self.registers[y]);
        self.registers[z] = u16::wrapping_rem(self.registers[x], self.registers[y]);
    }

    fn shln(&mut self, x: u8, n: u8) {
        self.flags.set_on_shl(self.registers[x], n.into());
        self.registers[x] = u16::wrapping_shl(self.registers[x], n.into());
    }

    fn shrn(&mut self, x: u8, n: u8) {
        self.flags.set_on_shr(self.registers[x], n.into());
        self.registers[x] = u16::wrapping_shr(self.registers[x], n.into());
    }

    fn sarn(&mut self, x: u8, n: u8) {
        // TODO
    }

    fn shlr(&mut self, x: u8, y: u8) {
        self.flags.set_on_shl(self.registers[x], self.registers[y]);
        self.registers[x] = u16::wrapping_shl(self.registers[x], self.registers[y].into());
    }

    fn shrr(&mut self, x: u8, y: u8) {
        self.flags.set_on_shl(self.registers[x], self.registers[y]);
        self.registers[x] = u16::wrapping_shl(self.registers[x], self.registers[y].into());
    }

    fn sarr(&mut self, x: u8, y: u8) {
        // TODO
    }

    fn push(&mut self, x: u8) {
        self.memory.write(self.stack_pointer, self.registers[x]);
        self.stack_pointer += 2;
    }

    fn pop(&mut self, x: u8) {
        self.stack_pointer -= 2;
        self.registers[x] = self.memory.read(self.stack_pointer);
    }

    fn pushall(&mut self) {
        for i in 0..16u8 {
            self.memory.write(self.stack_pointer, self.registers[i]);
            self.stack_pointer += 2;
        }
    }

    fn popall(&mut self) {
        for i in (0..16u8).rev() {
            self.stack_pointer -= 2;
            self.registers[i] = self.memory.read(self.stack_pointer);
        }
    }

    fn pushf(&mut self) {
        let flags: u8 = (&self.flags).into();
        self.memory.write(self.stack_pointer, flags);
        self.stack_pointer += 2;
    }

    fn popf(&mut self) {
        self.stack_pointer -= 2;
        let flags: u8 = self.memory.read(self.stack_pointer);
        self.flags = flags.into();
    }

    fn pali(&mut self, address: u16) {
        // TODO: Make palette a struct, and implement Read<I, Palette> for Memory.
        for i in 0..16u16 {
            let address = address + 3 * i;
            self.palette[i as usize].0 = self.memory.read(address);
            self.palette[i as usize].1 = self.memory.read(address + 1);
            self.palette[i as usize].2 = self.memory.read(address + 2);
        }
    }

    fn palr(&mut self, x: u8) {
        // TODO: Make palette a struct, and implement Read<I, Palette> for Memory.
        for i in 0..16u16 {
            let address = self.registers[x] + 3 * i;
            self.palette[i as usize].0 = self.memory.read(address);
            self.palette[i as usize].1 = self.memory.read(address + 1);
            self.palette[i as usize].2 = self.memory.read(address + 2);
        }
    }

    fn noti(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_not(immediate);
        self.registers[x] = !immediate;
    }

    fn notr1(&mut self, x: u8) {
        self.flags.set_on_not(self.registers[x]);
        self.registers[x] = !self.registers[x];
    }

    fn notr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_not(self.registers[y]);
        self.registers[x] = !self.registers[y];
    }

    fn negi(&mut self, x: u8, immediate: u16) {
        self.flags.set_on_neg(immediate);
        self.registers[x] = -(immediate as i16) as u16;
    }

    fn negr1(&mut self, x: u8) {
        self.flags.set_on_neg(self.registers[x]);
        self.registers[x] = -(self.registers[x] as i16) as u16;
    }

    fn negr2(&mut self, x: u8, y: u8) {
        self.flags.set_on_neg(self.registers[y]);
        self.registers[x] = -(self.registers[y] as i16) as u16;
    }
}
