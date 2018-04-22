#[derive(Default)]
pub struct Flags {
    pub c: bool,
    pub z: bool,
    pub o: bool,
    pub n: bool,
}

impl Flags {
    pub fn mark_for_add(&mut self, x: u16, y: u16) {
        // (result, self.o) = u16::overflowing_add(x, y);
        // self.c =
        // self.z = result == 0;
        // self.o = ((result as i32) < 0 && (x as i32) > 0 && (y as u32) > 0)
        //     || ((result as i32) > 0 && (x as i32) < 0 && (y as u32) < 0);
        // self.n = (result as i32) < 0;
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
