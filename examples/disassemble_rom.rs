extern crate byteorder;
extern crate chip16;

use byteorder::{LittleEndian, ReadBytesExt};
use chip16::{Instruction, Rom};
use std::env;
use std::fs::File;

fn main() {
    let filename = env::args().last().unwrap();
    let file = File::open(filename).unwrap();
    let rom = Rom::new(file).unwrap();

    let start_address = rom.start_address as usize;
    let mut reader = &rom.content[start_address..];
    let mut address = start_address;

    while let Ok(opcode) = reader.read_u32::<LittleEndian>() {
        if let Ok(instruction) = Instruction::decode(opcode) {
            println!("{:04x} {}", address, instruction.disassemble());
        } else {
            println!("{:04x}", address);
        }
        address += 4;
    }
}