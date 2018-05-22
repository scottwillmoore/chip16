RESEARCH.
--------------------

Various ways of implementing instructions.
https://play.rust-lang.org/?gist=3171b5b9c95e751f610198647b5ba054&version=stable&mode=debug

Various fixes to memory manipulation problems.
https://play.rust-lang.org/?gist=b0838deb612fa1ca298ba01bc00ceaee&version=stable&mode=debug

A way of defining CPU execution for multiple versions.
https://play.rust-lang.org/?gist=8817e7dbcde8160bbf6f953f98ff6199&version=stable&mode=debug

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