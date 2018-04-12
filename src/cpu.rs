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

    fn step(&self) {
        let opcode = 0 as u32;
        let instruction = Instruction::decode(opcode);

        // Decompose the instruction into bytes.
        let (b3, b2, b1, b0) = (
            (opcode & 0xFF000000 >> 0x18) as u8,
            (opcode & 0x00FF0000 >> 0x10) as u8,
            (opcode & 0x0000FF00 >> 0x08) as u8,
            (opcode & 0x000000FF) as u8,
        );

        // Decompose the instruction into nibbles.
        let (n7, n6, n5, n4, n3, n2, n1, n0) = (
            b0 & 0x0F,
            b0 & 0xF0,
            b1 & 0x0F,
            b1 & 0xF0,
            b2 & 0x0F,
            b2 & 0xF0,
            b3 & 0x0F,
            b3 & 0xF0,
        );

        // Decode the instruction.
        match instruction {
            NOP => {}
            _ => {}
        };
    }

    fn execute(&self, instruction: &Instruction) {}
}

