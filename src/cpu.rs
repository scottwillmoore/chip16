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
        let instruction = Instruction::new(opcode).expect("Instruction not found.");

        let h = (opcode & 0x000000FF >> 00) as u8;
        let l = (opcode & 0x0000FF00 >> 08) as u8;
        let z = (opcode & 0x00000F00 >> 12) as u8;
        let x = (opcode & 0x000F0000 >> 16) as u8;
        let y = (opcode & 0x00F00000 >> 20) as u8;

        self.execute(&instruction);
    }

    fn execute(&self, instruction: &Instruction) {
        match instruction {
            Instruction::NOP => {}
            _ => {}
        };
    }
}
