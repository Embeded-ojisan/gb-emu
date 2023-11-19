#[derive(Clone, Copy,Debug,Default)]
pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
}

impl Registers {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8;
    }

    pub fn zf(&self) -> bool {
        (self.f & 0b_1000_0000) > 0
    }

    pub fn set_zf(&mut self, zf: bool) {
        if zf {
            self.f |= 0b_1000_0000;
        } else {
            self.f &= 0b_0111_1111;
        }
    }
}