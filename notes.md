Use the existing mash16 emulator to test current implementation.
This could be modifying mash16 to create a memory dump after execution for comparison.

Implement all defined instructions.

Import enum variations into cpu module, NOP vs. Instruction::NOP.

Implement structs for memory, registers, palette, etc.
````rust
impl Memory<T, U> {
    fn get<T, U>(&self, index: T) -> U {}
    fn set<T, U>(&self, index: T, value: U) {}
}
````

Write better documentation for each module.

Re-export each internal module into a single module.

Exit gracefully, or handle errors that may be raised.

Create utilities for analysing cpu state.

Implement a c16 disassembler (perhaps in a seperate module... may import enums from this module).

Write unit tests for each instruction.

Write integration tests for expected execution of c16 binaries.

Consider execution of various versions of c16. How would this be handled?

Seperate binary and library from each other. Create implementation using pistion2d-graphics.
Possible folder names for the seperation could be core, graphics, wasm.

Done. Import instructions and flags into cpu module individually.
````rust
use instruction::Instruction;
Instruction::ADDR2;

use instruction::Instruction::*;
ADDR2;
````

Done. Change structs to use a more descriptive naming format.
````rust
struct Flags {
    carry: bool,
    zero: bool,
    overflow: bool,
    negative: bool,
}

struct Cpu {
    memory: Vec<u8>,
    registers: [u16; 16],
    flags: Flags,
    program_counter: u16,
    stack_pointer: u16,
    video_memory: Vec<u8>,
    palette: [Color; 16],
    background: u8,
    sprite_height: u8,
    sprite_width: u8,
    flip_horizontal: bool,
    flip_vertical: bool,
}
````