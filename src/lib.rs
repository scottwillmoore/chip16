// TODO: create flags type: for easy decoding, encoding of flags
// TODO: create separate memory type: for safe, convinient access of memory

enum Instruction {
    NOP,
    CLS,
    VBLNK,
    BGC { n: u8 },
    SPR { ll: u8, hh: u8 },
    DRW { y: u8, x: u8, ll: u8, hh: u8 },
}

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

// fetch-decode-execute

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            ..Default::default()
        }
    }

    fn step(&self) {
        let opcode = self.pc;

        match opcode {
            0x00_00_00_00 => {}
            0x01_00_00_00 => {}
            0x02_00_00_00 => {}
            _ => {}
        };

        // fetch

        // decode

        // execute
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_cpu() {
        let cpu = Cpu::new();
        cpu.step();
    }
}
