use instruction::Instruction;

#[derive(Default)]
struct Cpu {
    m: Vec<u8>,

    r: [u16; 16],
    pc: u16,
    sp: u16,
    f: u8,

    bg: u8,
    spritew: u8,
    spriteh: u8,
    hflip: bool,
    vlip: bool,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            ..Default::default()
        }
    }

    fn fetch(&self) -> u32 {
        0 as u32
    }

    fn step(&self) {
        let opcode = self.fetch();
        let instruction = Instruction::from_opcode(opcode).expect("Instruction not found.");

        self.execute(&instruction);
    }

    fn execute(&self, instruction: &Instruction) {
        match instruction {
            Instruction::NOP => {}
            _ => {}
        };
    }
}
