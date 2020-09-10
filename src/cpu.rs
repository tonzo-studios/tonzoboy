use std::path::Path;

use crate::register::{Register, Flag, Flag::*};
use crate::util::{make_word, lsb, msb, swap, rotate_left, rotate_right};
use crate::memory::Mmu;

pub struct Cpu {
    reg: Register,
    mmu: Mmu,
}

impl Cpu {
    pub fn new(rom_path: &Path) -> Self {
        let mut cpu = Self {
            reg: Register::new(),
            mmu: Mmu::new(),
        };
        cpu.mmu.load_rom(rom_path).expect("Failed to load the ROM");
        cpu
    }

    /// Read the next byte at the position of the PC register,
    /// and advance the PC register
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.read_byte_at(self.reg.pc);
        self.reg.pc += 1;
        byte
    }

    /// Read the next two bytes at the position of the PC register,
    /// the first one being the LSB, and advance the PC register
    pub fn fetch_word(&mut self) -> u16 {
        let byte1 = self.mmu.read_byte_at(self.reg.pc);
        let byte2 = self.mmu.read_byte_at(self.reg.pc + 1);
        self.reg.pc += 2;
        (byte1 as u16) | ((byte2 as u16) << 8)
    }

    pub fn run (&mut self) {
        let mut cycles = 0;
        cycles += self.step();
    }

    /// Read the next opcode from memory and execute it,
    /// returning the number of cycles that the instruction cost
    fn step(&mut self) -> u32 {
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
            0x7E => { self.reg.a = self.mmu.read_byte_at(self.reg.hl()); 8 },
            0x46 => { self.reg.b = self.mmu.read_byte_at(self.reg.hl()); 8 },
            0x4E => { self.reg.c = self.mmu.read_byte_at(self.reg.hl()); 8 },
            0x56 => { self.reg.d = self.mmu.read_byte_at(self.reg.hl()); 8 },
            0x5E => { self.reg.d = self.mmu.read_byte_at(self.reg.hl()); 8 },
            0x66 => { self.reg.h = self.mmu.read_byte_at(self.reg.hl()); 8 },
            0x6E => { self.reg.l = self.mmu.read_byte_at(self.reg.hl()); 8 },
            // LD (HL), r
            0x70 => { self.mmu.write_byte_at(self.reg.hl(), self.reg.b); 8 },
            0x71 => { self.mmu.write_byte_at(self.reg.hl(), self.reg.c); 8 },
            0x72 => { self.mmu.write_byte_at(self.reg.hl(), self.reg.d); 8 },
            0x73 => { self.mmu.write_byte_at(self.reg.hl(), self.reg.e); 8 },
            0x74 => { self.mmu.write_byte_at(self.reg.hl(), self.reg.h); 8 },
            0x75 => { self.mmu.write_byte_at(self.reg.hl(), self.reg.l); 8 },
            0x36 => { let v = self.fetch_byte(); self.mmu.write_byte_at(self.reg.hl(), v); 12 },
            // LD A,n
            0x0A => { self.reg.a = self.mmu.read_byte_at(self.reg.bc()); 8 },
            0x1A => { self.reg.a = self.mmu.read_byte_at(self.reg.de()); 8 },
            0xFA => { let addr = self.fetch_word(); self.reg.a = self.mmu.read_byte_at(addr); 16 },
            0x3E => { self.reg.a = self.fetch_byte(); 8 },
            // LD n,A
            0x47 => { self.reg.b = self.reg.a; 4 },
            0x4F => { self.reg.c = self.reg.a; 4 },
            0x57 => { self.reg.d = self.reg.a; 4 },
            0x5F => { self.reg.e = self.reg.a; 4 },
            0x67 => { self.reg.h = self.reg.a; 4 },
            0x6F => { self.reg.l = self.reg.a; 4 },
            0x02 => { self.mmu.write_byte_at(self.reg.bc(), self.reg.a); 8 },
            0x12 => { self.mmu.write_byte_at(self.reg.de(), self.reg.a); 8 },
            0x77 => { self.mmu.write_byte_at(self.reg.hl(), self.reg.a); 8 },
            0xEA => { let addr = self.fetch_word(); self.mmu.write_byte_at(addr, self.reg.a); 16 },
            // LD A,(C)
            0xF2 => { self.reg.a = self.mmu.read_byte_at(0xFF00 | self.reg.c as u16); 8 },
            // LD (C),A
            0xE2 => { self.mmu.write_byte_at(0xFF00 | self.reg.c as u16, self.reg.a); 8 }
            // LD A,(HLD)
            0x3A => { let hl = self.reg.hl(); self.reg.a = self.mmu.read_byte_at(hl); self.reg.set_hl(hl - 1); 8 },
            // LD (HLD),A
            0x32 => { let hl = self.reg.hl(); self.mmu.write_byte_at(hl, self.reg.a); self.reg.set_hl(hl - 1); 8 },
            // LD A,(HLI)
            0x2A => { let hl = self.reg.hl(); self.reg.a = self.mmu.read_byte_at(hl); self.reg.set_hl(hl + 1); 8 },
            // LD (HLI),A
            0x22 => { let hl = self.reg.hl(); self.mmu.write_byte_at(hl, self.reg.a); self.reg.set_hl(hl + 1); 8 },
            // LDH (n),A
            0xE0 => { let addr = 0xFF00 | self.fetch_byte() as u16; self.mmu.write_byte_at(addr, self.reg.a); 12 },
            // LDH A,(n)
            0xF0 => { let addr = 0xFF00 | self.fetch_byte() as u16; self.reg.a = self.mmu.read_byte_at(addr); 12 },
            // LD n,nn
            0x01 => { let v = self.fetch_word(); self.reg.set_bc(v); 12 },
            0x11 => { let v = self.fetch_word(); self.reg.set_de(v); 12 },
            0x21 => { let v = self.fetch_word(); self.reg.set_hl(v); 12 },
            0x31 => { self.reg.sp = self.fetch_word(); 12 },
            // LD SP,HL
            0xF9 => { self.reg.sp = self.reg.hl(); 12 },
            // LD (nn),SP
            0x08 => { let addr = self.fetch_word(); self.mmu.write_byte_at(addr, lsb(self.reg.sp)); self.mmu.write_byte_at(addr + 1, msb(self.reg.sp)); 20 },
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
            // ADD A,n
            0x87 => { self.add(self.reg.a); 4 },
            0x80 => { self.add(self.reg.b); 4 },
            0x81 => { self.add(self.reg.c); 4 },
            0x82 => { self.add(self.reg.d); 4 },
            0x83 => { self.add(self.reg.e); 4 },
            0x84 => { self.add(self.reg.h); 4 },
            0x85 => { self.add(self.reg.l); 4 },
            0x86 => { self.add(self.mmu.read_byte_at(self.reg.hl())); 8 },
            0xC6 => { let v = self.fetch_byte(); self.add(v); 8 },
            // ADC A,n
            0x8F => { self.adc(self.reg.a); 4 },
            0x88 => { self.adc(self.reg.b); 4 },
            0x89 => { self.adc(self.reg.c); 4 },
            0x8A => { self.adc(self.reg.d); 4 },
            0x8B => { self.adc(self.reg.e); 4 },
            0x8C => { self.adc(self.reg.h); 4 },
            0x8D => { self.adc(self.reg.l); 4 },
            0x8E => { self.adc(self.mmu.read_byte_at(self.reg.hl())); 8 },
            0xCE => { let v = self.fetch_byte(); self.adc(v); 8 },
            // SUB n
            0x97 => { self.sub(self.reg.a); 4 },
            0x90 => { self.sub(self.reg.b); 4 },
            0x91 => { self.sub(self.reg.c); 4 },
            0x92 => { self.sub(self.reg.d); 4 },
            0x93 => { self.sub(self.reg.e); 4 },
            0x94 => { self.sub(self.reg.h); 4 },
            0x95 => { self.sub(self.reg.l); 4 },
            0x96 => { self.sub(self.mmu.read_byte_at(self.reg.hl())); 8 },
            0xD6 => { let v = self.fetch_byte(); self.sub(v); 8 },
            // SBC A,n
            0x9F => { self.sbc(self.reg.a); 4 },
            0x98 => { self.sbc(self.reg.b); 4 },
            0x99 => { self.sbc(self.reg.c); 4 },
            0x9A => { self.sbc(self.reg.d); 4 },
            0x9B => { self.sbc(self.reg.e); 4 },
            0x9C => { self.sbc(self.reg.h); 4 },
            0x9D => { self.sbc(self.reg.l); 4 },
            0x9E => { self.sbc(self.mmu.read_byte_at(self.reg.hl())); 8 },
            // AND n
            0xA7 => { self.and(self.reg.a); 4 },
            0xA0 => { self.and(self.reg.b); 4 },
            0xA1 => { self.and(self.reg.c); 4 },
            0xA2 => { self.and(self.reg.d); 4 },
            0xA3 => { self.and(self.reg.e); 4 },
            0xA4 => { self.and(self.reg.h); 4 },
            0xA5 => { self.and(self.reg.l); 4 },
            0xA6 => { self.and(self.mmu.read_byte_at(self.reg.hl())); 8 },
            0xE6 => { let v = self.fetch_byte(); self.and(v); 8 },
            // OR n
            0xB7 => { self.or(self.reg.a); 4 },
            0xB0 => { self.or(self.reg.b); 4 },
            0xB1 => { self.or(self.reg.c); 4 },
            0xB2 => { self.or(self.reg.d); 4 },
            0xB3 => { self.or(self.reg.e); 4 },
            0xB4 => { self.or(self.reg.h); 4 },
            0xB5 => { self.or(self.reg.l); 4 },
            0xB6 => { self.or(self.mmu.read_byte_at(self.reg.hl())); 8 },
            0xF6 => { let v = self.fetch_byte(); self.or(v); 8 },
            // XOR n
            0xAF => { self.xor(self.reg.a); 4 },
            0xA8 => { self.xor(self.reg.b); 4 },
            0xA9 => { self.xor(self.reg.c); 4 },
            0xAA => { self.xor(self.reg.d); 4 },
            0xAB => { self.xor(self.reg.e); 4 },
            0xAC => { self.xor(self.reg.h); 4 },
            0xAD => { self.xor(self.reg.l); 4 },
            0xAE => { self.xor(self.mmu.read_byte_at(self.reg.hl())); 8 },
            0xEE => { let v = self.fetch_byte(); self.xor(v); 8 },
            // CP n
            0xBF => { self.cp(self.reg.a); 4 },
            0xB8 => { self.cp(self.reg.b); 4 },
            0xB9 => { self.cp(self.reg.c); 4 },
            0xBA => { self.cp(self.reg.d); 4 },
            0xBB => { self.cp(self.reg.e); 4 },
            0xBC => { self.cp(self.reg.h); 4 },
            0xBD => { self.cp(self.reg.l); 4 },
            0xBE => { self.cp(self.mmu.read_byte_at(self.reg.hl())); 8 },
            0xFE => { let v = self.fetch_byte(); self.cp(v); 8 },
            // INC n
            0x3C => { self.reg.a = self.inc(self.reg.a); 4 },
            0x04 => { self.reg.b = self.inc(self.reg.b); 4 },
            0x0C => { self.reg.c = self.inc(self.reg.c); 4 },
            0x14 => { self.reg.d = self.inc(self.reg.d); 4 },
            0x1C => { self.reg.e = self.inc(self.reg.e); 4 },
            0x24 => { self.reg.h = self.inc(self.reg.h); 4 },
            0x2C => { self.reg.l = self.inc(self.reg.l); 4 },
            0x34 => { let hl = self.reg.hl(); let v = self.inc(self.mmu.read_byte_at(hl)); self.mmu.write_byte_at(hl, v); 12 },
            // DEC n
            0x3D => { self.reg.a = self.dec(self.reg.a); 4 },
            0x05 => { self.reg.b = self.dec(self.reg.b); 4 },
            0x0D => { self.reg.c = self.dec(self.reg.c); 4 },
            0x15 => { self.reg.d = self.dec(self.reg.d); 4 },
            0x1D => { self.reg.e = self.dec(self.reg.e); 4 },
            0x25 => { self.reg.h = self.dec(self.reg.h); 4 },
            0x2D => { self.reg.l = self.dec(self.reg.l); 4 },
            0x35 => { let hl = self.reg.hl(); let v = self.dec(self.mmu.read_byte_at(hl)); self.mmu.write_byte_at(hl, v); 12 },
            // CB
            0xCB => { self.cb() },
            // CPL
            0x2F => { self.cpl(); 4 },
            // CCF
            0x3F => { self.ccf(); 4 },
            // SCF
            0x37 => { self.scf(); 4 },
            // NOP
            0x00 => { 4 },
            // Rotates (RLCA, RLA, RRCA, RRA)
            0x07 => { self.reg.a = self.rlc(self.reg.a); 4 },
            0x17 => { self.reg.a = self.rl(self.reg.a); 4 },
            0x0F => { self.reg.a = self.rrc(self.reg.a); 4 },
            0x1F => { self.reg.a = self.rr(self.reg.a); 4 },
            // Jumps
            0xC3 => { self.reg.pc = self.fetch_word(); 12 },
            0xC2 => { let addr = self.fetch_word(); self.jump_conditional(addr, Z, false); 12 },
            0xCA => { let addr = self.fetch_word(); self.jump_conditional(addr, Z, true); 12 },
            0xD2 => { let addr = self.fetch_word(); self.jump_conditional(addr, C, false); 12 },
            0xDA => { let addr = self.fetch_word(); self.jump_conditional(addr, C, true); 12 },
            0xE9 => { self.reg.pc = self.reg.hl(); 4 },
            0x18 => { self.reg.pc = self.reg.pc + (self.fetch_byte() as u16); 8 },
            0x20 => { let addr = self.fetch_byte() as u16; self.jump_conditional(addr, Z, false); 8 },
            0x28 => { let addr = self.fetch_byte() as u16; self.jump_conditional(addr, Z, true); 8 },
            0x30 => { let addr = self.fetch_byte() as u16; self.jump_conditional(addr, C, false); 8 },
            0x38 => { let addr = self.fetch_byte() as u16; self.jump_conditional(addr, C, true); 8 },
            _ => panic!("Unknown opcode {:x} found at address {:x}", opcode, self.reg.pc),
        }
    }

    /// Different interpretation of opcodes when they are preceded by the CB opcode
    fn cb(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        match opcode {
            // SWAP n
            0x37 => { self.swap(self.reg.a); 8 },
            0x30 => { self.swap(self.reg.b); 8 },
            0x31 => { self.swap(self.reg.c); 8 },
            0x32 => { self.swap(self.reg.d); 8 },
            0x33 => { self.swap(self.reg.e); 8 },
            0x34 => { self.swap(self.reg.h); 8 },
            0x35 => { self.swap(self.reg.l); 8 },
            0x36 => { let hl = self.reg.hl(); let v = self.swap(self.mmu.read_byte_at(hl)); self.mmu.write_byte_at(hl, v); 16 },
            // RLC n
            0x07 => { self.reg.a = self.rlc(self.reg.a); 8 },
            0x00 => { self.reg.b = self.rlc(self.reg.b); 8 },
            0x01 => { self.reg.c = self.rlc(self.reg.c); 8 },
            0x02 => { self.reg.d = self.rlc(self.reg.d); 8 },
            0x03 => { self.reg.e = self.rlc(self.reg.e); 8 },
            0x04 => { self.reg.h = self.rlc(self.reg.h); 8 },
            0x05 => { self.reg.l = self.rlc(self.reg.l); 8 },
            0x06 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.rlc(v); self.mmu.write_byte_at(hl, v); 16 },
            // RL n
            0x17 => { self.reg.a = self.rl(self.reg.a); 8 },
            0x10 => { self.reg.b = self.rl(self.reg.b); 8 },
            0x11 => { self.reg.c = self.rl(self.reg.c); 8 },
            0x12 => { self.reg.d = self.rl(self.reg.d); 8 },
            0x13 => { self.reg.e = self.rl(self.reg.e); 8 },
            0x14 => { self.reg.h = self.rl(self.reg.h); 8 },
            0x15 => { self.reg.l = self.rl(self.reg.l); 8 },
            0x16 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.rl(v); self.mmu.write_byte_at(hl, v); 16 },
            // RRC n
            0x0F => { self.reg.a = self.rrc(self.reg.a); 8 },
            0x08 => { self.reg.b = self.rrc(self.reg.b); 8 },
            0x09 => { self.reg.c = self.rrc(self.reg.c); 8 },
            0x0A => { self.reg.d = self.rrc(self.reg.d); 8 },
            0x0B => { self.reg.e = self.rrc(self.reg.e); 8 },
            0x0C => { self.reg.h = self.rrc(self.reg.h); 8 },
            0x0D => { self.reg.l = self.rrc(self.reg.l); 8 },
            0x0E => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.rrc(v); self.mmu.write_byte_at(hl, v); 16 },
            // RR n
            0x1F => { self.reg.a = self.rr(self.reg.a); 8 },
            0x18 => { self.reg.b = self.rr(self.reg.b); 8 },
            0x19 => { self.reg.c = self.rr(self.reg.c); 8 },
            0x1A => { self.reg.d = self.rr(self.reg.d); 8 },
            0x1B => { self.reg.e = self.rr(self.reg.e); 8 },
            0x1C => { self.reg.h = self.rr(self.reg.h); 8 },
            0x1D => { self.reg.l = self.rr(self.reg.l); 8 },
            0x1E => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.rr(v); self.mmu.write_byte_at(hl, v); 16 },
            // SLA n
            0x27 => { self.reg.a = self.sla(self.reg.a); 8 },
            0x20 => { self.reg.b = self.sla(self.reg.b); 8 },
            0x21 => { self.reg.c = self.sla(self.reg.c); 8 },
            0x22 => { self.reg.d = self.sla(self.reg.d); 8 },
            0x23 => { self.reg.e = self.sla(self.reg.e); 8 },
            0x24 => { self.reg.h = self.sla(self.reg.h); 8 },
            0x25 => { self.reg.l = self.sla(self.reg.l); 8 },
            0x26 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.sla(v); self.mmu.write_byte_at(hl, v); 16 },
            // SRA n
            0x2F => { self.reg.a = self.sra(self.reg.a); 8 },
            0x28 => { self.reg.b = self.sra(self.reg.b); 8 },
            0x29 => { self.reg.c = self.sra(self.reg.c); 8 },
            0x2A => { self.reg.d = self.sra(self.reg.d); 8 },
            0x2B => { self.reg.e = self.sra(self.reg.e); 8 },
            0x2C => { self.reg.h = self.sra(self.reg.h); 8 },
            0x2D => { self.reg.l = self.sra(self.reg.l); 8 },
            0x2E => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.sra(v); self.mmu.write_byte_at(hl, v); 16 },
            // SRL n
            0x3F => { self.reg.a = self.srl(self.reg.a); 8 },
            0x38 => { self.reg.b = self.srl(self.reg.b); 8 },
            0x39 => { self.reg.c = self.srl(self.reg.c); 8 },
            0x3A => { self.reg.d = self.srl(self.reg.d); 8 },
            0x3B => { self.reg.e = self.srl(self.reg.e); 8 },
            0x3C => { self.reg.h = self.srl(self.reg.h); 8 },
            0x3D => { self.reg.l = self.srl(self.reg.l); 8 },
            0x3E => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.srl(v); self.mmu.write_byte_at(hl, v); 16 },
            // BIT b,r
            0x40 => { self.bit(self.reg.b, 0); 8 },
            0x41 => { self.bit(self.reg.c, 0); 8 },
            0x42 => { self.bit(self.reg.d, 0); 8 },
            0x43 => { self.bit(self.reg.e, 0); 8 },
            0x44 => { self.bit(self.reg.h, 0); 8 },
            0x45 => { self.bit(self.reg.l, 0); 8 },
            0x46 => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 0); 16 },
            0x47 => { self.bit(self.reg.a, 0); 8 },
            0x48 => { self.bit(self.reg.b, 1); 8 },
            0x49 => { self.bit(self.reg.c, 1); 8 },
            0x4A => { self.bit(self.reg.d, 1); 8 },
            0x4B => { self.bit(self.reg.e, 1); 8 },
            0x4C => { self.bit(self.reg.h, 1); 8 },
            0x4D => { self.bit(self.reg.l, 1); 8 },
            0x4E => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 1); 16 },
            0x4F => { self.bit(self.reg.a, 1); 8 },
            0x50 => { self.bit(self.reg.b, 2); 8 },
            0x51 => { self.bit(self.reg.c, 2); 8 },
            0x52 => { self.bit(self.reg.d, 2); 8 },
            0x53 => { self.bit(self.reg.e, 2); 8 },
            0x54 => { self.bit(self.reg.h, 2); 8 },
            0x55 => { self.bit(self.reg.l, 2); 8 },
            0x56 => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 2); 16 },
            0x57 => { self.bit(self.reg.a, 2); 8 },
            0x58 => { self.bit(self.reg.b, 3); 8 },
            0x59 => { self.bit(self.reg.c, 3); 8 },
            0x5A => { self.bit(self.reg.d, 3); 8 },
            0x5B => { self.bit(self.reg.e, 3); 8 },
            0x5C => { self.bit(self.reg.h, 3); 8 },
            0x5D => { self.bit(self.reg.l, 3); 8 },
            0x5E => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 3); 16 },
            0x5F => { self.bit(self.reg.a, 3); 8 },
            0x60 => { self.bit(self.reg.b, 4); 8 },
            0x61 => { self.bit(self.reg.c, 4); 8 },
            0x62 => { self.bit(self.reg.d, 4); 8 },
            0x63 => { self.bit(self.reg.e, 4); 8 },
            0x64 => { self.bit(self.reg.h, 4); 8 },
            0x65 => { self.bit(self.reg.l, 4); 8 },
            0x66 => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 4); 16 },
            0x67 => { self.bit(self.reg.a, 4); 8 },
            0x68 => { self.bit(self.reg.b, 5); 8 },
            0x69 => { self.bit(self.reg.c, 5); 8 },
            0x6A => { self.bit(self.reg.d, 5); 8 },
            0x6B => { self.bit(self.reg.e, 5); 8 },
            0x6C => { self.bit(self.reg.h, 5); 8 },
            0x6D => { self.bit(self.reg.l, 5); 8 },
            0x6E => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 5); 16 },
            0x6F => { self.bit(self.reg.a, 5); 8 },
            0x70 => { self.bit(self.reg.b, 6); 8 },
            0x71 => { self.bit(self.reg.c, 6); 8 },
            0x72 => { self.bit(self.reg.d, 6); 8 },
            0x73 => { self.bit(self.reg.e, 6); 8 },
            0x74 => { self.bit(self.reg.h, 6); 8 },
            0x75 => { self.bit(self.reg.l, 6); 8 },
            0x76 => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 6); 16 },
            0x77 => { self.bit(self.reg.a, 6); 8 },
            0x78 => { self.bit(self.reg.b, 7); 8 },
            0x79 => { self.bit(self.reg.c, 7); 8 },
            0x7A => { self.bit(self.reg.d, 7); 8 },
            0x7B => { self.bit(self.reg.e, 7); 8 },
            0x7C => { self.bit(self.reg.h, 7); 8 },
            0x7D => { self.bit(self.reg.l, 7); 8 },
            0x7E => { self.bit(self.mmu.read_byte_at(self.reg.hl()), 7); 16 },
            0x7F => { self.bit(self.reg.a, 7); 8 },
            // RES b,r
            0x80 => { self.reg.b = self.set(self.reg.b, 0, false); 8 },
            0x81 => { self.reg.c = self.set(self.reg.c, 0, false); 8 },
            0x82 => { self.reg.d = self.set(self.reg.d, 0, false); 8 },
            0x83 => { self.reg.e = self.set(self.reg.e, 0, false); 8 },
            0x84 => { self.reg.h = self.set(self.reg.h, 0, false); 8 },
            0x85 => { self.reg.l = self.set(self.reg.l, 0, false); 8 },
            0x86 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0x87 => { self.reg.a = self.set(self.reg.a, 0, false); 8 },
            0x88 => { self.reg.c = self.set(self.reg.b, 1, false); 8 },
            0x89 => { self.reg.d = self.set(self.reg.c, 1, false); 8 },
            0x8A => { self.reg.e = self.set(self.reg.d, 1, false); 8 },
            0x8B => { self.reg.h = self.set(self.reg.e, 1, false); 8 },
            0x8C => { self.reg.l = self.set(self.reg.h, 1, false); 8 },
            0x8D => { self.reg.b = self.set(self.reg.l, 1, false); 8 },
            0x8E => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0x8F => { self.reg.a = self.set(self.reg.a, 1, false); 8 },
            0x90 => { self.reg.b = self.set(self.reg.b, 2, false); 8 },
            0x91 => { self.reg.c = self.set(self.reg.c, 2, false); 8 },
            0x92 => { self.reg.d = self.set(self.reg.d, 2, false); 8 },
            0x93 => { self.reg.e = self.set(self.reg.e, 2, false); 8 },
            0x94 => { self.reg.h = self.set(self.reg.h, 2, false); 8 },
            0x95 => { self.reg.l = self.set(self.reg.l, 2, false); 8 },
            0x96 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0x97 => { self.reg.a = self.set(self.reg.a, 2, false); 8 },
            0x98 => { self.reg.b = self.set(self.reg.b, 3, false); 8 },
            0x99 => { self.reg.c = self.set(self.reg.c, 3, false); 8 },
            0x9A => { self.reg.d = self.set(self.reg.d, 3, false); 8 },
            0x9B => { self.reg.e = self.set(self.reg.e, 3, false); 8 },
            0x9C => { self.reg.h = self.set(self.reg.h, 3, false); 8 },
            0x9D => { self.reg.l = self.set(self.reg.l, 3, false); 8 },
            0x9E => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0x9F => { self.reg.a = self.set(self.reg.a, 3, false); 8 },
            0xA0 => { self.reg.b = self.set(self.reg.b, 4, false); 8 },
            0xA1 => { self.reg.c = self.set(self.reg.c, 4, false); 8 },
            0xA2 => { self.reg.d = self.set(self.reg.d, 4, false); 8 },
            0xA3 => { self.reg.e = self.set(self.reg.e, 4, false); 8 },
            0xA4 => { self.reg.h = self.set(self.reg.h, 4, false); 8 },
            0xA5 => { self.reg.l = self.set(self.reg.l, 4, false); 8 },
            0xA6 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0xA7 => { self.reg.a = self.set(self.reg.a, 4, false); 8 },
            0xA8 => { self.reg.b = self.set(self.reg.b, 5, false); 8 },
            0xA9 => { self.reg.c = self.set(self.reg.c, 5, false); 8 },
            0xAA => { self.reg.d = self.set(self.reg.d, 5, false); 8 },
            0xAB => { self.reg.e = self.set(self.reg.e, 5, false); 8 },
            0xAC => { self.reg.h = self.set(self.reg.h, 5, false); 8 },
            0xAD => { self.reg.l = self.set(self.reg.l, 5, false); 8 },
            0xAE => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0xAF => { self.reg.a = self.set(self.reg.a, 5, false); 8 },
            0xB0 => { self.reg.b = self.set(self.reg.b, 6, false); 8 },
            0xB1 => { self.reg.c = self.set(self.reg.c, 6, false); 8 },
            0xB2 => { self.reg.d = self.set(self.reg.d, 6, false); 8 },
            0xB3 => { self.reg.e = self.set(self.reg.e, 6, false); 8 },
            0xB4 => { self.reg.h = self.set(self.reg.h, 6, false); 8 },
            0xB5 => { self.reg.l = self.set(self.reg.l, 6, false); 8 },
            0xB6 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0xB7 => { self.reg.a = self.set(self.reg.a, 6, false); 8 },
            0xB8 => { self.reg.b = self.set(self.reg.b, 7, false); 8 },
            0xB9 => { self.reg.c = self.set(self.reg.c, 7, false); 8 },
            0xBA => { self.reg.d = self.set(self.reg.d, 7, false); 8 },
            0xBB => { self.reg.e = self.set(self.reg.e, 7, false); 8 },
            0xBC => { self.reg.h = self.set(self.reg.h, 7, false); 8 },
            0xBD => { self.reg.l = self.set(self.reg.l, 7, false); 8 },
            0xBE => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, false); self.mmu.write_byte_at(hl, v); 16 },
            0xBF => { self.reg.a = self.set(self.reg.a, 7, false); 8 },
            // SET b,r
            0xC0 => { self.reg.b = self.set(self.reg.b, 0, true); 8 },
            0xC1 => { self.reg.c = self.set(self.reg.c, 0, true); 8 },
            0xC2 => { self.reg.d = self.set(self.reg.d, 0, true); 8 },
            0xC3 => { self.reg.e = self.set(self.reg.e, 0, true); 8 },
            0xC4 => { self.reg.h = self.set(self.reg.h, 0, true); 8 },
            0xC5 => { self.reg.l = self.set(self.reg.l, 0, true); 8 },
            0xC6 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xC7 => { self.reg.a = self.set(self.reg.a, 0, true); 8 },
            0xC8 => { self.reg.c = self.set(self.reg.b, 1, true); 8 },
            0xC9 => { self.reg.d = self.set(self.reg.c, 1, true); 8 },
            0xCA => { self.reg.e = self.set(self.reg.d, 1, true); 8 },
            0xCB => { self.reg.h = self.set(self.reg.e, 1, true); 8 },
            0xCC => { self.reg.l = self.set(self.reg.h, 1, true); 8 },
            0xCD => { self.reg.b = self.set(self.reg.l, 1, true); 8 },
            0xCE => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xCF => { self.reg.a = self.set(self.reg.a, 1, true); 8 },
            0xD0 => { self.reg.b = self.set(self.reg.b, 2, true); 8 },
            0xD1 => { self.reg.c = self.set(self.reg.c, 2, true); 8 },
            0xD2 => { self.reg.d = self.set(self.reg.d, 2, true); 8 },
            0xD3 => { self.reg.e = self.set(self.reg.e, 2, true); 8 },
            0xD4 => { self.reg.h = self.set(self.reg.h, 2, true); 8 },
            0xD5 => { self.reg.l = self.set(self.reg.l, 2, true); 8 },
            0xD6 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xD7 => { self.reg.a = self.set(self.reg.a, 2, true); 8 },
            0xD8 => { self.reg.b = self.set(self.reg.b, 3, true); 8 },
            0xD9 => { self.reg.c = self.set(self.reg.c, 3, true); 8 },
            0xDA => { self.reg.d = self.set(self.reg.d, 3, true); 8 },
            0xDB => { self.reg.e = self.set(self.reg.e, 3, true); 8 },
            0xDC => { self.reg.h = self.set(self.reg.h, 3, true); 8 },
            0xDD => { self.reg.l = self.set(self.reg.l, 3, true); 8 },
            0xDE => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xDF => { self.reg.a = self.set(self.reg.a, 3, true); 8 },
            0xE0 => { self.reg.b = self.set(self.reg.b, 4, true); 8 },
            0xE1 => { self.reg.c = self.set(self.reg.c, 4, true); 8 },
            0xE2 => { self.reg.d = self.set(self.reg.d, 4, true); 8 },
            0xE3 => { self.reg.e = self.set(self.reg.e, 4, true); 8 },
            0xE4 => { self.reg.h = self.set(self.reg.h, 4, true); 8 },
            0xE5 => { self.reg.l = self.set(self.reg.l, 4, true); 8 },
            0xE6 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xE7 => { self.reg.a = self.set(self.reg.a, 4, true); 8 },
            0xE8 => { self.reg.b = self.set(self.reg.b, 5, true); 8 },
            0xE9 => { self.reg.c = self.set(self.reg.c, 5, true); 8 },
            0xEA => { self.reg.d = self.set(self.reg.d, 5, true); 8 },
            0xEB => { self.reg.e = self.set(self.reg.e, 5, true); 8 },
            0xEC => { self.reg.h = self.set(self.reg.h, 5, true); 8 },
            0xED => { self.reg.l = self.set(self.reg.l, 5, true); 8 },
            0xEE => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xEF => { self.reg.a = self.set(self.reg.a, 5, true); 8 },
            0xF0 => { self.reg.b = self.set(self.reg.b, 6, true); 8 },
            0xF1 => { self.reg.c = self.set(self.reg.c, 6, true); 8 },
            0xF2 => { self.reg.d = self.set(self.reg.d, 6, true); 8 },
            0xF3 => { self.reg.e = self.set(self.reg.e, 6, true); 8 },
            0xF4 => { self.reg.h = self.set(self.reg.h, 6, true); 8 },
            0xF5 => { self.reg.l = self.set(self.reg.l, 6, true); 8 },
            0xF6 => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xF7 => { self.reg.a = self.set(self.reg.a, 6, true); 8 },
            0xF8 => { self.reg.b = self.set(self.reg.b, 7, true); 8 },
            0xF9 => { self.reg.c = self.set(self.reg.c, 7, true); 8 },
            0xFA => { self.reg.d = self.set(self.reg.d, 7, true); 8 },
            0xFB => { self.reg.e = self.set(self.reg.e, 7, true); 8 },
            0xFC => { self.reg.h = self.set(self.reg.h, 7, true); 8 },
            0xFD => { self.reg.l = self.set(self.reg.l, 7, true); 8 },
            0xFE => { let hl = self.reg.hl(); let mut v = self.mmu.read_byte_at(hl); v = self.set(v, 0, true); self.mmu.write_byte_at(hl, v); 16 },
            0xFF => { self.reg.a = self.set(self.reg.a, 7, true); 8 },
        }
    }

    /// Push a word into the stack memory, first the MSB and then the LSB,
    /// and decrement the stack pointer twice
    fn push(&mut self, word: u16) {
        self.reg.sp -= 1;
        self.mmu.write_byte_at(self.reg.sp, msb(word));
        self.reg.sp -= 1;
        self.mmu.write_byte_at(self.reg.sp, lsb(word));
    }

    /// Pop two bytes off the stack and return the resulting combined word,
    /// and increment the stack pointer twice
    fn pop(&mut self) -> u16 {
        let b1 = self.mmu.read_byte_at(self.reg.sp);
        self.reg.sp += 1;
        let b2 = self.mmu.read_byte_at(self.reg.sp);
        self.reg.sp += 1;
        make_word(b2, b1)
    }

    fn _add(&mut self, val: u8, carry: bool) {
        let c = carry as u8;
        let res = self.reg.a.wrapping_add(val).wrapping_add(c);
        self.reg.set_flag(Z, res == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, (self.reg.a & 0x0F) + (val & 0x0F) + c > 0x0F);
        self.reg.set_flag(C, (self.reg.a as u16) + (val as u16) + (c as u16) > 0xFF);
        self.reg.a = res;
    }

    fn add(&mut self, val: u8) {
        self._add(val, false);
    }

    fn adc(&mut self, val: u8) {
        self._add(val, self.reg.get_flag(C));
    }

    fn _sub(&mut self, val: u8, carry: bool) {
        let c = carry as u8;
        let res = self.reg.a.wrapping_sub(val).wrapping_sub(c);
        self.reg.set_flag(Z, res == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (self.reg.a & 0x0F) < (val & 0x0F) + c);
        self.reg.set_flag(C, (self.reg.a as u16) < (val as u16) + (c as u16));
    }

    fn sub(&mut self, val: u8) {
        self._sub(val, false);
    }

    fn sbc(&mut self, val: u8) {
        self._sub(val, self.reg.get_flag(C));
    }

    fn and(&mut self, val: u8) {
        self.reg.a &= val;
        self.reg.set_flag(Z, self.reg.a == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, true);
        self.reg.set_flag(C, false);
    }

    fn or(&mut self, val: u8) {
        self.reg.a |= val;
        self.reg.set_flag(Z, self.reg.a == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, false);
    }

    fn xor(&mut self, val: u8) {
        self.reg.a ^= val;
        self.reg.set_flag(Z, self.reg.a == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, false);
    }

    fn cp(&mut self, val: u8) {
        self.reg.set_flag(Z, self.reg.a == val);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (self.reg.a & 0x0F) < (val & 0x0F));
        self.reg.set_flag(C, self.reg.a < val);
    }

    /// Increment the value and return the results, also setting the corresponding flags
    /// as specified by the INC instruction
    fn inc(&mut self, val: u8) -> u8 {
        let res = val + 1;
        self.reg.set_flag(Z, res == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, (self.reg.a & 0x0F) + 1 > 0x0F);
        res
    }

    /// Decrement the value and return the results, also setting the corresponding flags
    /// as specified by the DEC instruction
    fn dec(&mut self, val: u8) -> u8 {
        let res = val - 1;
        self.reg.set_flag(Z, res == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (self.reg.a & 0x0F) < 1);
        res
    }

    fn swap(&mut self, val: u8) -> u8 {
        let res = swap(val);
        self.reg.set_flag(Z, res == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, false);
        res
    }

    fn cpl(&mut self) {
        self.reg.a = !self.reg.a;
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, true);
    }

    fn ccf(&mut self) {
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, !self.reg.get_flag(C));
    }

    fn scf(&mut self) {
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, true);
    }

    /// Updates flags Z, N and H for rotate and shift instructions using the
    /// given value as the 'result' of the instruction
    fn _rotate_shift_flag_update(&mut self, val: u8) {
        self.reg.set_flag(Z, val == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
    }

    fn rlc(&mut self, val: u8) -> u8 {
        self.reg.set_flag(C, val & 0x80 != 0);
        let res = rotate_left(val, 1);
        self._rotate_shift_flag_update(res);
        res
    }

    fn rl(&mut self, val: u8) -> u8 {
        let c = self.reg.get_flag(C) as u8;
        self.reg.set_flag(C, val & 0x80 != 0);
        let res = (val << 1) | c;
        self._rotate_shift_flag_update(res);
        res
    }

    fn rrc(&mut self, val: u8) -> u8 {
        self.reg.set_flag(C, val & 1 != 0);
        let res = rotate_right(val, 1);
        self._rotate_shift_flag_update(res);
        res
    }

    fn rr(&mut self, val: u8) -> u8 {
        let c = self.reg.get_flag(C) as u16;
        self.reg.set_flag(C, val & 1 != 0);
        let res = (((val as u16) >> 1) | (c << 8)) as u8;
        self._rotate_shift_flag_update(res);
        res
    }

    fn sla(&mut self, val: u8) -> u8 {
        self.reg.set_flag(C, val & 0x80 != 0);
        let res = val << 1;
        self._rotate_shift_flag_update(res);
        res
    }

    fn sra(&mut self, val: u8) -> u8 {
        self.reg.set_flag(C, val & 1 != 0);
        let res = (val >> 1) | (val & 0x80);
        self._rotate_shift_flag_update(res);
        res
    }

    fn srl(&mut self, val: u8) -> u8 {
        self.reg.set_flag(C, val & 1 != 0);
        let res = val >> 1;
        self._rotate_shift_flag_update(res);
        res
    }

    fn bit(&mut self, val: u8, b: u8) {
        let mask = 1 << b;
        self.reg.set_flag(Z, val & mask == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, true);
    }

    fn set(&mut self, val: u8, b: u8, state: bool) -> u8 {
        let mask = 1 << b;
        match state {
            true => val | mask,
            false => val & !mask
        }
    }

    /// Jump to the given address if the flag has the state passed as argument
    fn jump_conditional(&mut self, addr: u16, flag: Flag, state: bool) {
        if self.reg.get_flag(flag) == state {
            self.reg.pc = addr;
        }
    }
}
