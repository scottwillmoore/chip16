use instruction::{Condition, Instruction, Operation};
use memory::{Memory, VideoMemory};
use register::{Register, RegisterFile};

pub struct Cpu {
    memory: Memory,
    video_memory: VideoMemory,
    registers: RegisterFile,

    program_counter: u16,
    stack_pointer: u16,
    flags: u8,

    background_color: u8,
    sprite_width: u8,
    sprite_height: u8,
    vertical_flip: bool,
    horizontal_flip: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: Memory::new(),
            video_memory: VideoMemory::new(),
            registers: RegisterFile::new(),

            program_counter: 0,
            stack_pointer: 0,
            flags: 0,

            background_color: 0,
            sprite_width: 0,
            sprite_height: 0,
            vertical_flip: false,
            horizontal_flip: false,
        }
    }

    pub fn step(&mut self) {
        let data = self.memory.read_u32(self.program_counter);
        let instruction = Instruction::new(data);

        self.program_counter.checked_add(4).unwrap();

        self.execute(instruction);
    }

    pub fn execute(&mut self, instruction: Instruction) {
        let operation = instruction.decode_operation().unwrap();

        match operation {
            NOP => self.nop(),
            CLS => self.cls(),
        }
    }

    fn nop(&mut self) {}

    fn cls(&mut self) {
        self.background_color = 0;
        self.video_memory.reset();
    }

    fn ldir(&mut self, instruction: Instruction) {
        let register = self.registers.get_mut(instruction.x());
        *register = instruction.hhll();
    }
}
