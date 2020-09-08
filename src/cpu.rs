use crate::register::Register;

const MAX_MEM_SIZE: usize = 0xFFFF;

pub struct Cpu {
    register: Register,
    memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
            // TODO: Load from rom file
            memory: [0; MAX_MEM_SIZE],
        }
    }

    /// Read the next byte at the position of the PC register,
    /// and advance the PC register
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte_at(self.register.pc);
        self.register.pc += 1;
        byte
    }

    /// Read the next two bytes at the position of the PC register,
    /// the first one being the LSB, and advance the PC register
    pub fn fetch_word(&mut self) -> u16 {
        let byte1 = self.read_byte_at(self.register.pc);
        let byte2 = self.read_byte_at(self.register.pc + 1);
        self.register.pc += 2;
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
            0x06 => { self.register.b = self.fetch_byte(); 8 },
            0x0E => { self.register.c = self.fetch_byte(); 8 },
            0x16 => { self.register.d = self.fetch_byte(); 8 },
            0x1E => { self.register.e = self.fetch_byte(); 8 },
            0x26 => { self.register.h = self.fetch_byte(); 8 },
            0x2E => { self.register.l = self.fetch_byte(); 8 },
            // LD r1,r2
            0x7F => { 4 },
            0x78 => { self.register.a = self.register.b; 4 },
            0x79 => { self.register.a = self.register.c; 4 },
            0x7A => { self.register.a = self.register.d; 4 },
            0x7B => { self.register.a = self.register.e; 4 },
            0x7C => { self.register.a = self.register.h; 4 },
            0x7D => { self.register.a = self.register.l; 4 },
            0x40 => { 4 },
            0x41 => { self.register.b = self.register.c; 4 },
            0x42 => { self.register.b = self.register.d; 4 },
            0x43 => { self.register.b = self.register.e; 4 },
            0x44 => { self.register.b = self.register.h; 4 },
            0x45 => { self.register.b = self.register.l; 4 },
            0x48 => { self.register.c = self.register.b; 4 },
            0x49 => { 4 },
            0x4A => { self.register.c = self.register.d; 4 },
            0x4B => { self.register.c = self.register.e; 4 },
            0x4C => { self.register.c = self.register.h; 4 },
            0x4D => { self.register.c = self.register.l; 4 },
            0x50 => { self.register.d = self.register.b; 4 },
            0x51 => { self.register.d = self.register.c; 4 },
            0x52 => { 4 },
            0x53 => { self.register.d = self.register.e; 4 },
            0x54 => { self.register.d = self.register.h; 4 },
            0x55 => { self.register.d = self.register.l; 4 },
            0x58 => { self.register.e = self.register.b; 4 },
            0x59 => { self.register.e = self.register.c; 4 },
            0x5A => { self.register.e = self.register.d; 4 },
            0x5B => { 4 },
            0x5C => { self.register.e = self.register.h; 4 },
            0x5D => { self.register.e = self.register.l; 4 },
            0x60 => { self.register.h = self.register.b; 4 },
            0x61 => { self.register.h = self.register.c; 4 },
            0x62 => { self.register.h = self.register.d; 4 },
            0x63 => { self.register.h = self.register.e; 4 },
            0x64 => { 4 },
            0x65 => { self.register.h = self.register.l; 4 },
            0x68 => { self.register.l = self.register.b; 4 },
            0x69 => { self.register.l = self.register.c; 4 },
            0x6A => { self.register.l = self.register.d; 4 },
            0x6B => { self.register.l = self.register.e; 4 },
            0x6C => { self.register.l = self.register.h; 4 },
            0x6D => { 4 },
            // LD r, (HR)
            0x7E => { self.register.a = self.read_byte_at(self.register.hl()); 8 },
            0x46 => { self.register.b = self.read_byte_at(self.register.hl()); 8 },
            0x4E => { self.register.c = self.read_byte_at(self.register.hl()); 8 },
            0x56 => { self.register.d = self.read_byte_at(self.register.hl()); 8 },
            0x5E => { self.register.d = self.read_byte_at(self.register.hl()); 8 },
            0x66 => { self.register.h = self.read_byte_at(self.register.hl()); 8 },
            0x6E => { self.register.l = self.read_byte_at(self.register.hl()); 8 },
            // LD (HL), r
            0x70 => { self.write_byte_at(self.register.hl(), self.register.b); 8 },
            0x71 => { self.write_byte_at(self.register.hl(), self.register.c); 8 },
            0x72 => { self.write_byte_at(self.register.hl(), self.register.d); 8 },
            0x73 => { self.write_byte_at(self.register.hl(), self.register.e); 8 },
            0x74 => { self.write_byte_at(self.register.hl(), self.register.h); 8 },
            0x75 => { self.write_byte_at(self.register.hl(), self.register.l); 8 },
            0x36 => { let v = self.fetch_byte(); self.write_byte_at(self.register.hl(), v); 12 },
            // LD A,n
            0x0A => { self.register.a = self.read_byte_at(self.register.bc()); 8 },
            0x1A => { self.register.a = self.read_byte_at(self.register.de()); 8 },
            0xFA => { self.register.a = self.read_byte_at(self.fetch_word()); 16 },
            0x3E => { self.register.a = self.fetch_byte(); 8 },
            // LD n,A
            0x47 => { self.register.b = self.register.a; 4 },
            0x4F => { self.register.c = self.register.a; 4 },
            0x57 => { self.register.d = self.register.a; 4 },
            0x5F => { self.register.e = self.register.a; 4 },
            0x67 => { self.register.h = self.register.a; 4 },
            0x6F => { self.register.l = self.register.a; 4 },
            0x02 => { self.write_byte_at(self.register.bc(), self.register.a); 8 },
            0x12 => { self.write_byte_at(self.register.de(), self.register.a); 8 },
            0x77 => { self.write_byte_at(self.register.hl(), self.register.a); 8 },
            0xEA => { let addr = self.fetch_word(); self.write_byte_at(addr, self.register.a); 16 },
            // LD A,(C)
            0xF2 => { self.register.a = self.read_byte_at(0xFF00 | self.register.c as u16); 8 },
            // LD (C),A
            0xE2 => { self.write_byte_at(0xFF00 | self.register.c as u16, self.register.a); 8 }
            // LD A,(HLD)
            0x3A => { self.register.a = self.read_byte_at(self.register.hld()); 8 },
            // LD (HLD),A
            0x32 => { self.write_byte_at(self.register.hld(), self.register.a); 8 },
            // LD A,(HLI)
            0x2A => { self.register.a = self.read_byte_at(self.register.hli()); 8 },
            // LD (HLI),A
            0x32 => { self.write_byte_at(self.register.hli(), self.register.a); 8 },
            // LDH (n),A
            0xE0 => { self.write_byte_at(0xFF00 | self.fetch_byte() as u16, self.register.a); 12 },
            // LDH A,(n)
            0xF0 => { self.register.a = self.read_byte_at(0xFF00 | self.fetch_byte() as u16); 12 },
            // LD n,nn
            0x01 => { self.register.set_bc(self.fetch_word()); 12 },
            0x11 => { self.register.set_de(self.fetch_word()); 12 },
            0x21 => { self.register.set_hl(self.fetch_word()); 12 },
            0x31 => { self.register.sp = self.fetch_word(); 12 },
            _ => panic!("Unknown opcode found: 0x{:x}", opcode),
        }
    }
}
