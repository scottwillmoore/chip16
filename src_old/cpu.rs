use failure::Error;

use flags::Flags;
use instruction::{Condition, Instruction, Operation};
use memory::{Load, Memory, Store, VideoMemory};
use registers::Registers;
use rom::Rom;

use self::Condition::*;
use self::Operation::*;

#[derive(Default)]
pub struct Cpu {
    memory: Memory,
    registers: Registers,
    flags: Flags,
    program_counter: u16,
    stack_pointer: u16,
    video_memory: VideoMemory,
    background: u8,
    sprite_height: u8,
    sprite_width: u8,
    flip_horizontal: bool,
    flip_vertical: bool,
}

impl Cpu {
    pub fn new(rom: &Rom) -> Cpu {
        let mut memory = Memory::new();
        memory.store(0usize, &rom.content);

        Cpu {
            memory,
            ..Default::default()
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let instruction = self.fetch();

        // TODO: Use a checked add when incrementing the program counter.
        self.program_counter += 4;

        self.execute(instruction)
    }

    fn fetch(&self) -> Instruction {
        let data = self.memory.load(self.program_counter);
        Instruction::new(data)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), Error> {
        let operation = instruction.operation()?;
        // println!("{:?} {:?}", instruction, operation);

        match operation {
            NOP => self.op_nop(),
            JMPI => self.op_jmpi(&instruction),
            LDIR => self.op_ldir(&instruction),
        };

        Ok(())
    }

    fn op_nop(&mut self) {}

    fn op_jmpi(&mut self, instruction: &Instruction) {
        self.program_counter = instruction.hhll();
    }

    fn op_ldir(&mut self, instruction: &Instruction) {
        self.registers[instruction.x()] = instruction.hhll();
    }
}
