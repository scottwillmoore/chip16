extern crate byteorder;
extern crate chip16;

use byteorder::{LittleEndian, ReadBytesExt};
use chip16::{Instruction, Operation, Rom};
use std::env;
use std::fs::File;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename).unwrap();
    let rom = Rom::new(file).unwrap();

    let mut program = &rom.content[..];
    let mut address = rom.start_address;
    while let Ok(data) = program.read_u32::<LittleEndian>() {
        let instruction = Instruction::new(data);
        if let Some(operation) = instruction.decode_operation() {
            println!("{:04x}: {:08x} {:?}", address, data, operation);
        } else {
            println!("{:04x}: {:08x}", address, data);
        }
        address += 4;
    }
}
