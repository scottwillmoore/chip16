extern crate chip16;

use chip16::{Cpu, Memory, Rom};

fn main() {
    let filename = env::args().last().unwrap();
    let file = File::open(filename).unwrap();

    let rom = Rom::new(file);
    let mut memory = Memory::new(rom);

    let cpu = Cpu::new(memory);
    cpu.step();
}
