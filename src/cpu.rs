use crate::register::{Register, make_word, lsb, msb};

const MAX_MEM_SIZE: usize = 0xFFFF;

pub struct Cpu {
    reg: Register,
    memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Register::new(),
            // TODO: Load from rom file
            memory: [0; MAX_MEM_SIZE],
        }
    }

    /// Read the next byte at the position of the PC register,
    /// and advance the PC register
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte_at(self.reg.pc);
        self.reg.pc += 1;
        byte
    }

    /// Read the next two bytes at the position of the PC register,
    /// the first one being the LSB, and advance the PC register
    pub fn fetch_word(&mut self) -> u16 {
        let byte1 = self.read_byte_at(self.reg.pc);
        let byte2 = self.read_byte_at(self.reg.pc + 1);
        self.reg.pc += 2;
        (byte1 as u16) | ((byte2 as u16) << 8)
    }

    pub fn read_byte_at(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte_at(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value
    }

    /// Read the next opcode from memory and execute it,
    /// returning the number of cycles that the instruction cost
    fn step(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        match opcode {
            // LD nn,n
            0x06 => { self.reg.b = self.fetch_byte(); 8 },
            0x0E => { self.reg.c = self.fetch_byte(); 8 },
            0x16 => { self.reg.d = self.fetch_byte(); 8 },
            0x1E => { self.reg.e = self.fetch_byte(); 8 },
            0x26 => { self.reg.h = self.fetch_byte(); 8 },
            0x2E => { self.reg.l = self.fetch_byte(); 8 },
            // LD r1,r2
            0x7F => { 4 },
            0x78 => { self.reg.a = self.reg.b; 4 },
            0x79 => { self.reg.a = self.reg.c; 4 },
            0x7A => { self.reg.a = self.reg.d; 4 },
            0x7B => { self.reg.a = self.reg.e; 4 },
            0x7C => { self.reg.a = self.reg.h; 4 },
            0x7D => { self.reg.a = self.reg.l; 4 },
            0x40 => { 4 },
            0x41 => { self.reg.b = self.reg.c; 4 },
            0x42 => { self.reg.b = self.reg.d; 4 },
            0x43 => { self.reg.b = self.reg.e; 4 },
            0x44 => { self.reg.b = self.reg.h; 4 },
            0x45 => { self.reg.b = self.reg.l; 4 },
            0x48 => { self.reg.c = self.reg.b; 4 },
            0x49 => { 4 },
            0x4A => { self.reg.c = self.reg.d; 4 },
            0x4B => { self.reg.c = self.reg.e; 4 },
            0x4C => { self.reg.c = self.reg.h; 4 },
            0x4D => { self.reg.c = self.reg.l; 4 },
            0x50 => { self.reg.d = self.reg.b; 4 },
            0x51 => { self.reg.d = self.reg.c; 4 },
            0x52 => { 4 },
            0x53 => { self.reg.d = self.reg.e; 4 },
            0x54 => { self.reg.d = self.reg.h; 4 },
            0x55 => { self.reg.d = self.reg.l; 4 },
            0x58 => { self.reg.e = self.reg.b; 4 },
            0x59 => { self.reg.e = self.reg.c; 4 },
            0x5A => { self.reg.e = self.reg.d; 4 },
            0x5B => { 4 },
            0x5C => { self.reg.e = self.reg.h; 4 },
            0x5D => { self.reg.e = self.reg.l; 4 },
            0x60 => { self.reg.h = self.reg.b; 4 },
            0x61 => { self.reg.h = self.reg.c; 4 },
            0x62 => { self.reg.h = self.reg.d; 4 },
            0x63 => { self.reg.h = self.reg.e; 4 },
            0x64 => { 4 },
            0x65 => { self.reg.h = self.reg.l; 4 },
            0x68 => { self.reg.l = self.reg.b; 4 },
            0x69 => { self.reg.l = self.reg.c; 4 },
            0x6A => { self.reg.l = self.reg.d; 4 },
            0x6B => { self.reg.l = self.reg.e; 4 },
            0x6C => { self.reg.l = self.reg.h; 4 },
            0x6D => { 4 },
            // LD r, (HR)
            0x7E => { self.reg.a = self.read_byte_at(self.reg.hl()); 8 },
            0x46 => { self.reg.b = self.read_byte_at(self.reg.hl()); 8 },
            0x4E => { self.reg.c = self.read_byte_at(self.reg.hl()); 8 },
            0x56 => { self.reg.d = self.read_byte_at(self.reg.hl()); 8 },
            0x5E => { self.reg.d = self.read_byte_at(self.reg.hl()); 8 },
            0x66 => { self.reg.h = self.read_byte_at(self.reg.hl()); 8 },
            0x6E => { self.reg.l = self.read_byte_at(self.reg.hl()); 8 },
            // LD (HL), r
            0x70 => { self.write_byte_at(self.reg.hl(), self.reg.b); 8 },
            0x71 => { self.write_byte_at(self.reg.hl(), self.reg.c); 8 },
            0x72 => { self.write_byte_at(self.reg.hl(), self.reg.d); 8 },
            0x73 => { self.write_byte_at(self.reg.hl(), self.reg.e); 8 },
            0x74 => { self.write_byte_at(self.reg.hl(), self.reg.h); 8 },
            0x75 => { self.write_byte_at(self.reg.hl(), self.reg.l); 8 },
            0x36 => { let v = self.fetch_byte(); self.write_byte_at(self.reg.hl(), v); 12 },
            // LD A,n
            0x0A => { self.reg.a = self.read_byte_at(self.reg.bc()); 8 },
            0x1A => { self.reg.a = self.read_byte_at(self.reg.de()); 8 },
            0xFA => { let addr = self.fetch_word(); self.reg.a = self.read_byte_at(addr); 16 },
            0x3E => { self.reg.a = self.fetch_byte(); 8 },
            // LD n,A
            0x47 => { self.reg.b = self.reg.a; 4 },
            0x4F => { self.reg.c = self.reg.a; 4 },
            0x57 => { self.reg.d = self.reg.a; 4 },
            0x5F => { self.reg.e = self.reg.a; 4 },
            0x67 => { self.reg.h = self.reg.a; 4 },
            0x6F => { self.reg.l = self.reg.a; 4 },
            0x02 => { self.write_byte_at(self.reg.bc(), self.reg.a); 8 },
            0x12 => { self.write_byte_at(self.reg.de(), self.reg.a); 8 },
            0x77 => { self.write_byte_at(self.reg.hl(), self.reg.a); 8 },
            0xEA => { let addr = self.fetch_word(); self.write_byte_at(addr, self.reg.a); 16 },
            // LD A,(C)
            0xF2 => { self.reg.a = self.read_byte_at(0xFF00 | self.reg.c as u16); 8 },
            // LD (C),A
            0xE2 => { self.write_byte_at(0xFF00 | self.reg.c as u16, self.reg.a); 8 }
            // LD A,(HLD)
            0x3A => { let hld = self.reg.hld(); self.reg.a = self.read_byte_at(hld); 8 },
            // LD (HLD),A
            0x32 => { let hld = self.reg.hld(); self.write_byte_at(hld, self.reg.a); 8 },
            // LD A,(HLI)
            0x2A => { let hli = self.reg.hli(); self.reg.a = self.read_byte_at(hli); 8 },
            // LD (HLI),A
            0x22 => { let hli = self.reg.hli(); self.write_byte_at(hli, self.reg.a); 8 },
            // LDH (n),A
            0xE0 => { let addr = 0xFF00 | self.fetch_byte() as u16; self.write_byte_at(addr, self.reg.a); 12 },
            // LDH A,(n)
            0xF0 => { let addr = 0xFF00 | self.fetch_byte() as u16; self.reg.a = self.read_byte_at(addr); 12 },
            // LD n,nn
            0x01 => { let v = self.fetch_word(); self.reg.set_bc(v); 12 },
            0x11 => { let v = self.fetch_word(); self.reg.set_de(v); 12 },
            0x21 => { let v = self.fetch_word(); self.reg.set_hl(v); 12 },
            0x31 => { self.reg.sp = self.fetch_word(); 12 },
            // LD SP,HL
            0xF9 => { self.reg.sp = self.reg.hl(); 12 },
            // LD (nn),SP
            0x08 => { let addr = self.fetch_word(); self.write_byte_at(addr, lsb(self.reg.sp)); self.write_byte_at(addr + 1, msb(self.reg.sp)); 20 },
            // PUSH nn
            0xF5 => { self.push(self.reg.af()); 16 },
            0xC5 => { self.push(self.reg.bc()); 16 },
            0xD5 => { self.push(self.reg.de()); 16 },
            0xE5 => { self.push(self.reg.hl()); 16 },
            // POP nn
            0xF1 => { let v = self.pop(); self.reg.set_af(v); 12 },
            0xC1 => { let v = self.pop(); self.reg.set_bc(v); 12 },
            0xD1 => { let v = self.pop(); self.reg.set_de(v); 12 },
            0xE1 => { let v = self.pop(); self.reg.set_hl(v); 12 },
            _ => panic!("Unknown opcode found: 0x{:x}", opcode),
        }
    }

    /// Push a word into the stack memory, first the MSB and then the LSB,
    /// and decrement the stack pointer twice
    fn push(&mut self, word: u16) {
        self.reg.sp -= 1;
        self.write_byte_at(self.reg.sp, msb(word));
        self.reg.sp -= 1;
        self.write_byte_at(self.reg.sp, lsb(word));
    }

    /// Pop two bytes off the stack and return the resulting combined word,
    /// and increment the stack pointer twice
    fn pop(&mut self) -> u16 {
        let b1 = self.read_byte_at(self.reg.sp);
        self.reg.sp += 1;
        let b2 = self.read_byte_at(self.reg.sp);
        self.reg.sp += 2;
        make_word(b2, b1)
    }
}
