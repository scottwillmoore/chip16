#[derive(Default)]
pub struct Flags {
    pub c: bool,
    pub z: bool,
    pub o: bool,
    pub n: bool,
}

impl Flags {
    pub fn set_on_add(&mut self, a: u16, b: u16) {
        let (_, c) = a.overflowing_add(b);
        let (signed_result, o) = (a as i16).overflowing_add(b as i16);

        self.c = c;
        self.z = signed_result == 0;
        self.o = o;
        self.n = signed_result < 0;
    }

    pub fn set_on_sub(&mut self, a: u16, b: u16) {
        let (_, c) = a.overflowing_sub(b);
        let (signed_result, o) = (a as i16).overflowing_sub(b as i16);

        self.c = c;
        self.z = signed_result == 0;
        self.o = o;
        self.n = signed_result < 0;
    }

    pub fn set_on_and(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16) & (b as i16);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_or(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16) | (b as i16);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_xor(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16) ^ (b as i16);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_mul(&mut self, a: u16, b: u16) {
        let (_, c) = a.overflowing_mul(b);
        let (signed_result, _) = (a as i16).overflowing_mul(b as i16);

        self.c = c;
        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_div(&mut self, a: u16, b: u16) {
        let (_, c) = a.overflowing_div(b);
        let (signed_result, _) = (a as i16).overflowing_div(b as i16);

        self.c = c;
        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_mod(&mut self, a: u16, b: u16) {
        // TODO
        let signed_result = (a as i16) % (b as i16);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_rem(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16).wrapping_rem(b as i16);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_shl(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16).wrapping_shl(b as u32);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_shr(&mut self, a: u16, b: u16) {
        let signed_result = a.wrapping_shr(b as u32) as i16;

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_sar(&mut self, a: u16, b: u16) {
        let signed_result = (a as i16).wrapping_shr(b as u32);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }

    pub fn set_on_not(&mut self, a: u16) {
        let result = !a;

        self.z = result == 0;
        self.n = result < 0;
    }

    pub fn set_on_neg(&mut self, a: u16) {
        let signed_result = -(a as i16);

        self.z = signed_result == 0;
        self.n = signed_result < 0;
    }
}

impl From<u8> for Flags {
    fn from(x: u8) -> Flags {
        Flags {
            c: (x >> 1) & 1 == 1,
            z: (x >> 2) & 1 == 1,
            o: (x >> 6) & 1 == 1,
            n: (x >> 7) & 1 == 1,
        }
    }
}

impl<'a> From<&'a Flags> for u8 {
    fn from(f: &'a Flags) -> u8 {
        ((f.c as u8) << 1) & ((f.z as u8) << 2) & ((f.o as u8) << 6) & ((f.n as u8) << 7)
    }
}
