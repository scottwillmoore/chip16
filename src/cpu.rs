use instruction::Instruction;

#[derive(Default)]
struct Flags {
    c: bool,
    z: bool,
    o: bool,
    n: bool,
}

struct Controller {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    select: bool,
    start: bool,
    a: bool,
    b: bool,
}

#[derive(Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default)]
struct Cpu {
    m: Vec<u8>,
    r: [u16; 16],
    f: u8,
    pc: u16,
    sp: u16,

    g: Vec<u8>,
    p: [Color; 16],
    bg: u8,
    sh: u8,
    sw: u8,
    fh: bool,
    fv: bool,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            m: Vec::with_capacity(65_536),
            g: Vec::with_capacity(320 * 240),
            ..Default::default()
        }
    }

    fn fetch(&self) -> u32 {
        0 as u32
    }

    fn step(&mut self) {
        let opcode = self.fetch();
        let instruction = Instruction::from_opcode(opcode).expect("Instruction not found.");

        self.execute(&instruction);
    }

    fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::NOP => {}
            Instruction::CLS => {
                self.bg = 0u8;
                self.g.clear();
            }
            Instruction::VBLNK => {
                // TODO
            }
            Instruction::BGC { n } => {
                self.bg = n;
            }
            Instruction::SPR { ll, hh } => {
                self.sw = ll;
                self.sh = hh;
            }
            _ => {}
        };
    }
}
