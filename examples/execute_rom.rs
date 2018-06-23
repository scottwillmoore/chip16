extern crate byteorder;
extern crate chip16;

use byteorder::{LittleEndian, ReadBytesExt};
use chip16::{Cpu, Instruction, Rom};
use std::env;
use std::fs::File;

fn main() {
    let filename = env::args().last().unwrap();
    let file = File::open(filename).unwrap();
    let rom = Rom::new(file).unwrap();

    // let start_address = rom.start_address as usize;
    // let mut reader = &rom.content[start_address..];
    // let mut address = start_address;

    let mut cpu = Cpu::new(&rom);
    while let Ok(_) = cpu.step() {}
}
