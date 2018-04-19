trait Address {}
trait Memory {}

impl Address for u16 {}

impl Memory for Vec<u8> {}

// PROBLEMS:
// TODO: Map out use case on paper, design nice API to use to solve the problem.
// TODO: Explore full range of instructions before deciding on an appropriate method.

// A u8 needs to be a usize (e.g. to index a register).
// A u16 needs to be a usize (e.g. to index into the memory).
// ---
// SOLUTION: Implement a trait which allows easier indexing into registers, memory.
// SOLUTION: Create a new Memory struct which allows indexing by u16.
// SOLUTION: Create a new Registers struct which allows indexing by u8.
// SOLUTION: Suck it up, it is a very simple cast to perform.

// A u16 needs to be written to memory (e.g. to write the pc into memory).
// ---
// SOLUTION: Implement a trait which allows easier writing into memory.

// Two u8's need to be converted into a u16 (e.g. to convert hh and ll into hhll).
// --
// SOLUTION: Eliminate situations that create this problem.
//           Modify the Instruction enum to accept a u16 instead.
//           This would require tweaking the Instruction definitions, may not always work.
// SOLUTION: Create a function to convert two u8's into a u16.
//           e.g. (u8, u8) -> u16

// trait GetSet<T, U> {
//     fn get<T, U>(&self, index: T) -> U;

//     fn set<T, U>(&self, index: T, value: U);
// }

// impl Memory {
//     fn new() -> Memory {
//         Memory { m: vec![] }
//     }

//     fn get<T, U>(&self, index: T) -> U {}

//     fn set<T, U>(&self, index: T, value: U) {}
// }

// impl Memory2 {
//     fn read_u8(&self, index: usize) -> u8 {}
//     fn read_u16(&self, index: usize) -> u16 {}
//     fn write_u8(&self, index: usize, value: u8) {}
//     fn write_u16(&self, index: usize, value: u16) {}
// }

#[cfg(test)]
mod test {}
