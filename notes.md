Implement all defined instructions.

Import enum variations into cpu module, NOP vs. Instruction::NOP.

Change structs to use a more descriptive naming format.
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