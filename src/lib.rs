// TODO: create flags type: for easy decoding, encoding of flags
// TODO: create separate memory type: for safe, convinient access of memory

enum Instruction {
    NOP,
    CLS,
    VBLNK,
    BGC,
    SPR,
    DRW0,
    DRW1,
    RND,
    FLIP0,
    FLIP1,
    FLIP2,
    FLIP3,
    SND0,
    SND1,
    SND2,
    SND3,
    SNP,
    SNG,
    JMP0,
    JMC,
    JX,
    JME,
    CALL0,
    RET,
    JMP1,
    CX,
    CALL1,
    LDI0,
    LDI1,
    LDM0,
    LDM1,
    MOV,
    STM0,
    STM1,
    ADDI,
    ADD0,
    ADD1,
    SUBI,
    SUB0,
    SUB1,
    CMPI,
    CMP,
    ANDI,
    AND0,
    AND1,
    TSTI,
    TST,
    ORI,
    OR0,
    OR1,
    XORI,
    XOR0,
    XOR1,
    MULI,
    MUL0,
    MUL1,
    DIVI,
    DIV0,
    DIV1,
    MODI,
    MOD0,
    MOD1,
    REMI,
    REM0,
    REM1,
    SHL0,
    SHR0,
    SAL0,
    SAR0,
    SHL1,
    SHR1,
    SAL1,
    SAR1,
    PUSH,
    POP,
    PUSHALL,
    POPALL,
    PUSHF,
    POPF,
    PAL0,
    PAL1,
    NOTI,
    NOT0,
    NOT1,
    NEGI,
    NEG0,
    NEG1,
}

impl Instruction {
    fn decode(opcode: u32) -> Result<Instruction, &'static str> {
        let b3 = (opcode & 0xFF000000 >> 24) as u8;

        match b3 {
            _ => Err(""),
        }
    }
}

#[derive(Default)]
struct Cpu {
    m: Vec<u8>,

    r: [u16; 16],
    pc: u16,
    sp: u16,
    f: u8,

    bg: u8,
    spritew: u8,
    spriteh: u8,
    hflip: bool,
    vlip: bool,
}

// fetch-decode-execute

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            ..Default::default()
        }
    }

    fn step(&self) {
        let opcode = 0 as u32;
        let instruction = Instruction::decode(opcode);

        // Decompose the instruction into bytes.
        let (b3, b2, b1, b0) = (
            (opcode & 0xFF000000 >> 0x18) as u8,
            (opcode & 0x00FF0000 >> 0x10) as u8,
            (opcode & 0x0000FF00 >> 0x08) as u8,
            (opcode & 0x000000FF) as u8,
        );

        // Decompose the instruction into nibbles.
        let (n7, n6, n5, n4, n3, n2, n1, n0) = (
            b0 & 0x0F,
            b0 & 0xF0,
            b1 & 0x0F,
            b1 & 0xF0,
            b2 & 0x0F,
            b2 & 0xF0,
            b3 & 0x0F,
            b3 & 0xF0,
        );

        // Decode the instruction.
        match b3 {
            0x00 => {}
            0x01 => {}
            0x02 => {}
            0x03 => {}
            0x04 => {}
            0x05 => {}
            0x06 => {}
            0x07 => {}
            0x08 => {}
            0x09 => {}
            0x0A => {}
            0x0B => {}
            0x0C => {}
            0x0D => {}
            0x0E => {}
            _ => {}
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_cpu() {
        let cpu = Cpu::new();
        cpu.step();
    }
}
