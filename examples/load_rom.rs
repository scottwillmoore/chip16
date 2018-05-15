extern crate chip16;

use chip16::{Cpu, Memory, Rom};
use std::env;
use std::fs::File;

fn main() {
    let filename = env::args().last().unwrap();
    let mut file = File::open(filename).unwrap();

    let rom = Rom::read(&mut file);
    // let mut memory = Memory::new(rom);

    let mut cpu = Cpu::new();
    cpu.step();
}
