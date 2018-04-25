use self::Condition::*;

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
            0x0 => Ok(Z),
            0x1 => Ok(NZ),
            0x2 => Ok(N),
            0x3 => Ok(NN),
            0x4 => Ok(P),
            0x5 => Ok(O),
            0x6 => Ok(NO),
            0x7 => Ok(A),
            0x8 => Ok(AE),
            0x9 => Ok(B),
            0xA => Ok(BE),
            0xB => Ok(G),
            0xC => Ok(GE),
            0xD => Ok(L),
            0xE => Ok(LE),
            _ => Err("Failed to decode byte into a condition."),
        }
    }
}
