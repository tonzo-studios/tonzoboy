use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

const MEM_BANK_SIZE: usize = 0xFFFF + 1; // 16 ^ 4

pub enum ColorMode {
    Color,
    NoColor,
}

/// Memory Management Unit (MMU)
pub struct Mmu {
    // TODO: Break into into memory sections (working ram, vram, oam, etc)
    // TODO: Add support for more memory banks
    memory: [u8; MEM_BANK_SIZE],
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            memory: [0; MEM_BANK_SIZE],
        }
    }

    pub fn load_rom(&mut self, rom_path: &Path) -> Result<(), io::Error> {
        let mut buffer = Vec::new();
        File::open(&rom_path)?.read_to_end(&mut buffer)?;
        // Copy data into memory
        for (i, byte) in buffer.iter().enumerate() {
            self.memory[i] = *byte;
        }
        Ok(())
    }

    pub fn read_byte_at(&self, address: u16) -> u8 {
        let mut address = address as usize;
        match address {
            0xE000..=0xFE00 => address -= 0x2000,
            _ => {}
        }
        self.memory[address]
    }

    pub fn write_byte_at(&mut self, address: u16, value: u8) {
        let mut address = address as usize;
        match address {
            0xE000..=0xFE00 => address -= 0x2000,
            _ => {}
        }
        self.memory[address] = value;
    }

    /// Return the game title as specified in the ROM data
    pub fn game_title(&self) -> String {
        let mut title = String::new();
        for i in 0x134..0x143 {
            title.push(self.read_byte_at(i) as char);
        }
        title
    }

    /// Return whether the ROM is for Game Boy Color or normal Game Boy
    pub fn color_mode(&self) -> ColorMode {
        match self.read_byte_at(0x143) {
            0x80 => ColorMode::Color,
            _ => ColorMode::NoColor,
        }
    }
}
