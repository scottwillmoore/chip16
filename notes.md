PLAN.
--------------------

Choose an extremely small subset of the instructions and implement them.
Then choose a really basic rom (maze, pong, etc) and implement all instructions needed for that rom.
Write tests for the instructions as you go.
Ensure emulator works for these roms, including graphics, etc.
Slowly expand to include the entire set of instructions.

RESEARCH.
--------------------

Various ways of implementing instructions.
https://play.rust-lang.org/?gist=3171b5b9c95e751f610198647b5ba054&version=stable&mode=debug

Various fixes to memory manipulation problems.
https://play.rust-lang.org/?gist=b0838deb612fa1ca298ba01bc00ceaee&version=stable&mode=debug

A way of defining CPU execution for multiple versions.
https://play.rust-lang.org/?gist=8817e7dbcde8160bbf6f953f98ff6199&version=stable&mode=debug

Using macros to extract u8, u16, etc from larger primitives.
https://play.rust-lang.org/?gist=22d1949a9b80d81567c023a2451c00a4&version=stable&mode=debug

Using generics to extract u8, u16 from a u32 type. Jesus, it is a little intense!
https://play.rust-lang.org/?gist=3bfc165ff3bc1fb1cd0ad9502c35c57c&version=nightly&mode=debug
https://play.rust-lang.org/?gist=ac281d21a1fcdbd5b9a29c5fbe4a7d36&version=nightly&mode=debug

This could be a better architecture for encoding, decode, assembling, disassembling instructions.
Note. Could this be made more generic, without having to manually implement each instruction?
Eg. When assembling instructions, we will have to be smarter...
https://play.rust-lang.org/?gist=090c7c9b1b32261797da030ea932c58e&version=stable&mode=debug

Structure and naming convetions used for CPU instructions.
`enum Operation` or `enum Opcode` or `enum Kind`.
Determined by the upper byte of the instruction. What does this instruction do.
`enum Format` or `enum Arguments`.
Based on the opcode, we can determine the format of the arguments.
````rust
enum Operation {
    ADD,
    JMP,
}

enum Format {
    RR,
    I,
}

struct Instruction {
    operation: Operation,
    format: Format,
    data: u32,
}

struct Instruction(u32);

impl Instruction {
    fn operation(&self) -> Operation {}
    fn format(&self) -> Format {}

    ...

    // Is there a safer way to wrap these functions.
    // E.g. Encapsulate them in the Format enum?
    fn hhll(&self) -> u16 {}
    fn x(&self) -> u8 {}
}
````
Also... Take into account the closures can often be optimized away...

TASKS.
--------------------

Todo: Continue to work on errors throught Rom struct.
I believe there are many errors that are not handled with proper context.
This could be easily discovered/fixed by ensuring proper test coverage.
Create some real dodgy roms for testing purposes.

Todo: Include opcode information in errors. E.g. What specific opcode failed?

Todo: Use the existing mash16 emulator to test current implementation.
This could be modifying mash16 to create a memory dump after execution for comparison.
This could be used as a easy form of creating a suite of unit/integration tests.

Todo: Write better documentation for public API.

Todo: Ensure functions exit gracefully and use errors. This should appear in public API.

Todo: Create a c16 diassembler. This should be in a seperate module.

Maybe: Explore using macros to generate the chip16 instruction parsing, disassembly, assembly?

Maybe: Implement a way to be faithful to c16 versions.
E.g. Do not execute version 1.3 instructions for a 1.1 c16 binary.

Maybe: Create a c16 assembler.

COMPLETED.
--------------------

Proposal: Rename 'address' in the (some/all) of the instructions to immediate.
Especially for the LD and ST instructions.
You could use address, immediate and indirect, etc. Be more descriptive.
Done.

Done. Implement structs for memory, registers, palette, etc.
````rust
impl Memory<T, U> {
    fn get<T, U>(&self, index: T) -> U {}
    fn set<T, U>(&self, index: T, value: U) {}
}
````

Done. Re-export each internal module into a single module.

Done. Import enum variations into cpu module, NOP vs. Instruction::NOP.

Done. Revise instruction names. Consider more of the following...
````
ADD_R2
LDR_I
LDR_M
STM_R
JMP
... etc
````

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