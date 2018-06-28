use flags::Flags;
use instruction::{Condition, Instruction, Operation};
use memory::{Memory, VideoMemory};
use rand::{thread_rng, Rng, ThreadRng};
use register::{Register, RegisterFile};

pub struct Cpu {
    memory: Memory,
    video_memory: VideoMemory,
    registers: RegisterFile,

    program_counter: u16,
    stack_pointer: u16,
    flags: Flags,

    background_color: u8,
    sprite_width: u8,
    sprite_height: u8,
    vertical_flip: bool,
    horizontal_flip: bool,

    wait_vblnk: bool,

    rng: ThreadRng,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: Memory::new(),
            video_memory: VideoMemory::new(),
            registers: RegisterFile::new(),

            program_counter: 0,
            stack_pointer: 0,
            flags: Flags::new(),

            background_color: 0,
            sprite_width: 0,
            sprite_height: 0,
            vertical_flip: false,
            horizontal_flip: false,

            wait_vblnk: false,

            rng: thread_rng(),
        }
    }

    pub fn step(&mut self) {
        let data = self.memory.read_u32(self.program_counter);
        let instruction = Instruction::new(data);

        self.program_counter.checked_add(4).unwrap();

        self.execute(instruction);
    }

    pub fn test(&mut self, condition: Condition) -> bool {
        match condition {
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
        let operation = instruction.decode_operation().unwrap();

        let execution = match operation {
            NOP => Cpu::nop,
            CLS => Cpu::cls,
            VBLNK => Cpu::vblnk,
            SPR => Cpu::spr,
        };

        execution(self, instruction);
    }

    fn nop(&mut self, instruction: Instruction) {}

    fn cls(&mut self, instruction: Instruction) {
        self.background_color = 0;
        self.video_memory.reset();
    }

    fn vblnk(&mut self, instruction: Instruction) {
        // TODO: Implement for maze.c16.
        self.wait_vblnk = true;
    }

    fn bgc(&mut self, instruction: Instruction) {}

    fn spr(&mut self, instruction: Instruction) {
        self.sprite_width = instruction.ll();
        self.sprite_height = instruction.hh();
    }

    fn drwi(&mut self, instruction: Instruction) {}
    fn drwr(&mut self, instruction: Instruction) {}

    fn rnd(&mut self, instruction: Instruction) {
        let x = self.registers.get_mut(instruction.x());
        *x = self.rng.gen_range(0, instruction.hhll());
    }

    fn flip(&mut self, instruction: Instruction) {}
    fn snd0(&mut self, instruction: Instruction) {}
    fn snd1(&mut self, instruction: Instruction) {}
    fn snd2(&mut self, instruction: Instruction) {}
    fn snd3(&mut self, instruction: Instruction) {}
    fn snp(&mut self, instruction: Instruction) {}
    fn sng(&mut self, instruction: Instruction) {}

    fn jmpi(&mut self, instruction: Instruction) {
        self.program_counter = instruction.hhll();
    }

    fn jmc(&mut self, instruction: Instruction) {}

    fn jx(&mut self, instruction: Instruction) {
        let condition = instruction.decode_condition().unwrap();
        if self.test(condition) {
            self.program_counter = instruction.hhll();
        }
    }

    fn jme(&mut self, instruction: Instruction) {
        if self.registers.get(instruction.x()) == self.registers.get(instruction.y()) {
            self.program_counter = instruction.hhll();
        }
    }

    fn calli(&mut self, instruction: Instruction) {}
    fn ret(&mut self, instruction: Instruction) {}
    fn jmpr(&mut self, instruction: Instruction) {}
    fn cx(&mut self, instruction: Instruction) {}
    fn callr(&mut self, instruction: Instruction) {}

    fn ldir(&mut self, instruction: Instruction) {
        let x = self.registers.get_mut(instruction.x());
        *x = instruction.hhll();
    }

    fn ldis(&mut self, instruction: Instruction) {}
    fn ldmi(&mut self, instruction: Instruction) {}
    fn ldmr(&mut self, instruction: Instruction) {}
    fn mov(&mut self, instruction: Instruction) {}
    fn stmi(&mut self, instruction: Instruction) {}
    fn stmr(&mut self, instruction: Instruction) {}

    fn addi(&mut self, instruction: Instruction) {
        let x = self.registers.get_mut(instruction.x());
        *x = self.flags.add(*x, instruction.hhll());
    }

    fn addr2(&mut self, instruction: Instruction) {
        let y = *self.registers.get_mut(instruction.y());
        let x = self.registers.get_mut(instruction.x());
        *x = self.flags.add(*x, y);
    }

    fn addr3(&mut self, instruction: Instruction) {
        let x = *self.registers.get_mut(instruction.x());
        let y = *self.registers.get_mut(instruction.y());
        let z = self.registers.get_mut(instruction.z());
        *z = self.flags.add(x, y);
    }

    fn subi(&mut self, instruction: Instruction) {}
    fn subr2(&mut self, instruction: Instruction) {}
    fn subr3(&mut self, instruction: Instruction) {}
    fn cmpi(&mut self, instruction: Instruction) {}
    fn cmpr(&mut self, instruction: Instruction) {}
    fn andi(&mut self, instruction: Instruction) {}
    fn andr2(&mut self, instruction: Instruction) {}
    fn andr3(&mut self, instruction: Instruction) {}
    fn tsti(&mut self, instruction: Instruction) {}
    fn tstr(&mut self, instruction: Instruction) {}
    fn ori(&mut self, instruction: Instruction) {}
    fn orr2(&mut self, instruction: Instruction) {}
    fn orr3(&mut self, instruction: Instruction) {}
    fn xori(&mut self, instruction: Instruction) {}
    fn xorr2(&mut self, instruction: Instruction) {}
    fn xorr3(&mut self, instruction: Instruction) {}
    fn muli(&mut self, instruction: Instruction) {}
    fn mulr2(&mut self, instruction: Instruction) {}
    fn mulr3(&mut self, instruction: Instruction) {}
    fn divi(&mut self, instruction: Instruction) {}
    fn divr2(&mut self, instruction: Instruction) {}
    fn divr3(&mut self, instruction: Instruction) {}
    fn modi(&mut self, instruction: Instruction) {}
    fn modr2(&mut self, instruction: Instruction) {}
    fn modr3(&mut self, instruction: Instruction) {}
    fn remi(&mut self, instruction: Instruction) {}
    fn remr2(&mut self, instruction: Instruction) {}
    fn remr3(&mut self, instruction: Instruction) {}
    fn shln(&mut self, instruction: Instruction) {}
    fn shrn(&mut self, instruction: Instruction) {}
    fn sarn(&mut self, instruction: Instruction) {}
    fn shlr(&mut self, instruction: Instruction) {}
    fn shrr(&mut self, instruction: Instruction) {}
    fn sarr(&mut self, instruction: Instruction) {}
    fn push(&mut self, instruction: Instruction) {}
    fn pop(&mut self, instruction: Instruction) {}
    fn pushall(&mut self, instruction: Instruction) {}
    fn popall(&mut self, instruction: Instruction) {}
    fn pushf(&mut self, instruction: Instruction) {}
    fn popf(&mut self, instruction: Instruction) {}
    fn pali(&mut self, instruction: Instruction) {}
    fn palr(&mut self, instruction: Instruction) {}
    fn noti(&mut self, instruction: Instruction) {}
    fn notr1(&mut self, instruction: Instruction) {}
    fn notr2(&mut self, instruction: Instruction) {}
    fn negi(&mut self, instruction: Instruction) {}
    fn negr1(&mut self, instruction: Instruction) {}
    fn negr2(&mut self, instruction: Instruction) {}
}
