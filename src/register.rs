pub struct Register {
    pub a: u8,
    f: u8, // flags register
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Register {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    // Some instructions use the A, B, C, D, E, H, L registers
    // as 16-bit registers by combining them in the following
    // manner: AF, BC, DE, HL
    pub fn af(&self) -> u16 {
        get_combined_register(self.a, self.f)
    }

    pub fn bc(&self) -> u16 {
        get_combined_register(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        get_combined_register(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        get_combined_register(self.l, self.l)
    }

    /// Return register HL and decrement HL
    pub fn hld(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl - 1);
        hl
    }

    /// Return register HL and increment HL
    pub fn hli(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl + 1);
        hl
    }

    pub fn set_af(&mut self, value: u16) {
        let (v1, v2) = split_combined_register(value);
        self.a = v1;
        self.f = v2;
    }

    pub fn set_bc(&mut self, value: u16) {
        let (v1, v2) = split_combined_register(value);
        self.b = v1;
        self.c = v2;
    }

    pub fn set_de(&mut self, value: u16) {
        let (v1, v2) = split_combined_register(value);
        self.d = v1;
        self.e = v2;
    }

    pub fn set_hl(&mut self, value: u16) {
        let (v1, v2) = split_combined_register(value);
        self.h = v1;
        self.l = v2;
    }
}

#[inline]
fn get_combined_register(r1: u8, r2: u8) -> u16 {
    ((r1 as u16) << 8) | (r2 as u16)
}

#[inline]
fn split_combined_register(value: u16) -> (u8, u8) {
    ((value >> 8) as u8, (value & 0x00FF) as u8)
}

pub enum Flag {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000,
}
