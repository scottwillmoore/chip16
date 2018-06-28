#[derive(Default)]
pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl Flags {
    pub fn set_on_add(&mut self, a: u16, b: u16) {
        let (_, carry) = a.overflowing_add(b);
        let (signed_result, overflow) = (a as i16).overflowing_add(b as i16);

        self.carry = carry;
        self.zero = signed_result == 0;
        self.overflow = overflow;
        self.negative = signed_result < 0;
    }

    pub fn set_on_sub(&mut self, a: u16, b: u16) {
        let (_, carry) = a.overflowing_sub(b);
        let (signed_result, overflow) = (a as i16).overflowing_sub(b as i16);

        self.carry = carry;
        self.zero = signed_result == 0;
        self.overflow = overflow;
        self.negative = signed_result < 0;
    }

    pub fn set_on_and(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16) & (b as i16);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_or(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16) | (b as i16);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_xor(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16) ^ (b as i16);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_mul(&mut self, a: u16, b: u16) {
        let (_, carry) = a.overflowing_mul(b);
        let (signed_result, _) = (a as i16).overflowing_mul(b as i16);

        self.carry = carry;
        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_div(&mut self, a: u16, b: u16) {
        let (_, carry) = a.overflowing_div(b);
        let (signed_result, _) = (a as i16).overflowing_div(b as i16);

        self.carry = carry;
        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_mod(&mut self, a: u16, b: u16) {
        // TODO
        let signed_result = (a as i16) % (b as i16);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_rem(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16).wrapping_rem(b as i16);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_shl(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16).wrapping_shl(b as u32);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_shr(&mut self, a: u16, b: u16) {
        let signed_result = a.wrapping_shr(b as u32) as i16;

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_sar(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16).wrapping_shr(b as u32);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_not(&mut self, a: u16) {
        let signed_result = !a as i16;

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }

    pub fn set_on_neg(&mut self, a: u16) {
        let signed_result = -(a as i16);

        self.zero = signed_result == 0;
        self.negative = signed_result < 0;
    }
}

impl From<u8> for Flags {
    fn from(x: u8) -> Flags {
        Flags {
            carry: (x >> 1) & 1 == 1,
            zero: (x >> 2) & 1 == 1,
            overflow: (x >> 6) & 1 == 1,
            negative: (x >> 7) & 1 == 1,
        }
    }
}

impl<'a> From<&'a Flags> for u8 {
    fn from(flags: &'a Flags) -> u8 {
        ((flags.carry as u8) << 1) & ((flags.zero as u8) << 2) & ((flags.overflow as u8) << 6)
            & ((flags.negative as u8) << 7)
    }
}
