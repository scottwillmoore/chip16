#[derive(Debug, PartialEq)]
pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            carry: false,
            zero: false,
            overflow: false,
            negative: false,
        }
    }

    // NOTE: This is NOT an cpu operation, but it is used by addi, addr2 and addr3.
    pub fn add(&mut self, a: u16, b: u16) -> u16 {
        let (_, carry) = u16::overflowing_add(a, b);
        let (signed_result, overflow) = i16::overflowing_add(a as i16, b as i16);

        self.carry = carry;
        self.zero = signed_result == 0;
        self.overflow = overflow;
        self.negative = signed_result < 0;

        signed_result as u16
    }
}
