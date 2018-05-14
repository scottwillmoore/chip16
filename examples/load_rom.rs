extern crate chip16;

use std::env;
use std::fs::File;
use chip16::{Cpu, Memory, Rom};

fn main() {
    let filename = env::args().last().unwrap();
    let mut file = File::open(filename).unwrap();

    let rom = Rom::read(&mut file);
    let mut memory = Memory::new(rom);

    let cpu = Cpu::new(memory);
    cpu.step();
}
