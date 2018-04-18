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
struct Memory {
    m: Vec<u8>,
}

trait Set<T, U> {
    fn set(&mut self, index: T) -> &mut U;
}

// u8 -> u16
// u16

impl Set<u8, u8> for Memory {
    fn set(&mut self, index: u8) -> &mut u8 {
        &mut self.m[0]
    }
}

// impl Set<u8, u16> for Memory {
//     fn set(&mut self, index: u8) -> &mut u16 {
//         &mut self.m[0]
//     }
// }

impl Memory {
    fn test(&mut self) {
        // let r = self.set(0);
        // *r = 0u8;
        *self.set(0) = 0u8;
        // THIS DOESN'T WORK FOR u16, because we can't return a mutable reference with a Vec<u8>...
        // self.set(0, 0u16) <- This will work however... At the cost of being ugly?
        // TODO: Investigate more elegant, but safe methods of addressing memory with u8 and u16.
    }
    // fn set<T, U>(&mut self, value: T) -> U
    // where
    //     T: ,
    // {
    // }
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

// TODO: Break up instructions into smaller steps.
// e.g. Converting between u8, u16 and usize.
// e.g. Type casting.

// TODO: Create a Memory struct for getting, setting memory.
// e.g. Indexing with u8, u16, multiple u8.
// e.g. Extracting u16.

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
            Instruction::DRWI { y, x, ll, hh } => {
                // TODO
            }
            Instruction::DRWR { y, x, z } => {
                // TODO
            }
            Instruction::RND { x, ll, hh } => {
                // TODO
            }
            Instruction::FLIP { fh, fv } => {
                self.fh = fh;
                self.fv = fv;
            }
            Instruction::SND0 {} => {
                // TODO
            }
            Instruction::SND1 { ll, hh } => {
                //TODO
            }
            Instruction::SND2 { ll, hh } => {
                // TODO
            }
            Instruction::SND3 { ll, hh } => {
                // TODO
            }
            Instruction::SNP { x, ll, hh } => {
                // TODO
            }
            Instruction::SNG { ad, sr, vt } => {
                // TODO
            }
            Instruction::JMPI { ll, hh } => {
                self.pc = (ll as u16) & ((hh as u16) << 0x8);
            }
            Instruction::JMC { ll, hh } => {
                // TODO
            }
            Instruction::JX { x, ll, hh } => {
                // TODO
            }
            Instruction::JME { y, x, ll, hh } => {
                // self.r.set(x, self.r.get(y))
                // self.r[x] = self.r[y] -> custom Index<u8>
                if self.r[x as usize] == self.r[y as usize] {
                    // self.pc = to_u16(ll, hh)
                    self.pc = (ll as u16) & (hh as u16) << 0x8;
                }
            }
            Instruction::CALLI { ll, hh } => {
                let sp = self.sp as usize;
                // self.memory.set(self.sp, self.pc)
                // self.memory.set(self.sp, self.pc)
                self.m[sp] = (self.pc & 0x00FF) as u8;
                self.m[sp + 1] = (self.pc & 0xFF00 >> 0x8) as u8;

                self.sp += 2;

                let ll = ll as u16;
                let hh = hh as u16;
                // self.pc = to_u16(ll, hh)
                self.pc = ll & (hh << 0x8);

                // self.m[self.sp as usize] = (self.pc & 0x00FF) as u8;
                // self.m[(self.sp + 1) as usize] = (self.pc & 0xFF00 >> 0x8) as u8;
                // self.sp += 2;
                // self.pc = (ll as u16) & ((hh as u16) << 0x8);
            }
            Instruction::RET => {
                self.sp -= 2;
                self.pc = (self.m[self.sp as usize] as u16)
                    & ((self.m[(self.sp + 1) as usize] as u16) << 0x8);
            }
            Instruction::JMPR { x } => {
                self.pc = x as u16;
            }
            Instruction::CX { x, ll, hh } => {
                // TODO
            }
            Instruction::CALLR { x } => {
                self.m[self.sp as usize] = (self.pc & 0x00FF) as u8;
                self.m[(self.sp + 1) as usize] = (self.pc & 0xFF00 >> 0x8) as u8;
                self.sp += 2;
                self.pc = self.r[x as usize];
            }
            Instruction::LDIR { x, ll, hh } => {
                self.r[x as usize] = (ll as u16) & ((hh as u16) << 0x8);
            }
            Instruction::LDIS { ll, hh } => {
                self.sp = (ll as u16) & ((hh as u16) << 0x8);
            }
            Instruction::LDMI { x, ll, hh } => {
                self.m[ll as usize] = (self.r[x as usize] & 0x00FF) as u8;
                self.m[hh as usize] = (self.r[x as usize] & 0xFF00 >> 0x8) as u8;
            }
            Instruction::LDMR { y, x } => {
                self.r[x as usize] =
                    (self.m[y as usize] as u16) & ((self.m[(y + 1) as usize] as u16) << 0x8);
            }
            Instruction::MOV { y, x } => {
                self.r[x as usize] = self.r[y as usize];
            }
            _ => {
                // TODO
            }
        };
    }
}
