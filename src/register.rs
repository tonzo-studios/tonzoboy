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
        make_word(self.a, self.f)
    }

    pub fn bc(&self) -> u16 {
        make_word(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        make_word(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        make_word(self.l, self.l)
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
        self.a = msb(value);
        self.f = lsb(value);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = msb(value);
        self.c = lsb(value);
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = msb(value);
        self.e = lsb(value);
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = msb(value);
        self.l = lsb(value);
    }

    /// Return whether the flag is set to true or false
    pub fn get_flag(&self, flag: Flag) -> bool {
        self.f & (flag as u8) != 0
    }

    /// Set the flag to the given state
    pub fn set_flag(&mut self, flag: Flag, state: bool) {
        let flag = flag as u8;
        match state {
            true => self.f |= flag,
            false => self.f &= !flag,
        }
        self.f &= 0xF0;
    }
}

/// Creates a word from two bytes
#[inline]
pub fn make_word(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | (lsb as u16)
}

/// Return the LSB (least significant byte) of the given word
#[inline]
pub fn lsb(word: u16) -> u8 {
    (word & 0x00FF) as u8
}

/// Return the MSB (most significant byte) of the given word
#[inline]
pub fn msb(word: u16) -> u8 {
    (word >> 8) as u8
}

pub enum Flag {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000,
}
