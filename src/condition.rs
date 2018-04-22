pub enum Condition {
    Z,
    NZ,
    N,
    NN,
    P,
    O,
    NO,
    A,
    AE,
    B,
    BE,
    G,
    GE,
    L,
    LE,
}

impl Condition {
    pub fn decode(x: u8) -> Result<Condition, &'static str> {
        match x {
            0x0 => Ok(Condition::Z),
            0x1 => Ok(Condition::NZ),
            0x2 => Ok(Condition::N),
            0x3 => Ok(Condition::NN),
            0x4 => Ok(Condition::P),
            0x5 => Ok(Condition::O),
            0x6 => Ok(Condition::NO),
            0x7 => Ok(Condition::A),
            0x8 => Ok(Condition::AE),
            0x9 => Ok(Condition::B),
            0xA => Ok(Condition::BE),
            0xB => Ok(Condition::G),
            0xC => Ok(Condition::GE),
            0xD => Ok(Condition::L),
            0xE => Ok(Condition::LE),
            _ => Err("Failed to decode byte into a condition."),
        }
    }
}
