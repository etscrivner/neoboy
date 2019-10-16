use super::*;
use super::cartridge::{Cartridge};

// GameBoy contains 65,536 bytes of addressabel memory. While the whole space
// is addressable, many of the addresses in this space are unavailable for
// various reasons. The layout is as follows:
//
//     -- 0x0000 - 0x3FFF (Cartridge ROM, Non-switchable ROM bank)
//     -- 0x4000 - 0x7FFF (Cartridge ROMx, Switchable ROM bank)
//     -- 0x8000 - 0x9FFF (VRAM, switchable (0-1) in CGB)
//     -- 0xA000 - 0xBFFF (Cartridge RAM, if available)
//     -- 0xC000 - 0xCFFF (Work RAM or WRAM)
//     -- 0xD000 - 0xDFFF (Work RAM, Switchable (1-7) in CGB)
//     -- 0xE000 - 0xFDFF (WRAM Mirror of 0xC000 and 0xCFFF)
//          * DMG same as CGB except for some flashcarts.
//          * CGB reads and writes are mirrored to 0xC000 - 0xDDFF (WRAM).
//     -- 0xFEAO - 0xFEFF (Unused)
//          * DMG writes ignored, reads return 0x00
//          * CGB contains 48 bytes at 0xFEA0 - 0xFECF, 0xFEC0-0xFEF0 mirror
//            writes and reads across 4 16-byte areas at FEC0, FED0, FEE0, and
//            FEF0.
//     -- 0xFF00 - 0xFF7F (memory mapped I/O Registers)
//     -- 0xFF80 - 0xFFFE (internal CPU RAM)
//     -- 0xFFFF (interrupt enable [IE] flag, all 8-bits are R/W)

/// Size of Gameboy main system memory in bytes.
pub const GAMEBOY_MEMORY_SIZE_BYTES: usize = 0x10000;

/// Represents the total memory contained in the GameBoy
pub struct Memory {
    pub data: [u8; GAMEBOY_MEMORY_SIZE_BYTES],
    pub cartridge: Box<Cartridge>
}

// TODO: Figure out how to handle I/O DMA addresses (callbacks?)
// TODO: Figure out if we can allocate less memory since cartridge accounts for most.
impl Memory {
    /// Allocate new GameBoy main system memory and initializes various areas.
    pub fn new(cartridge: Box<Cartridge>) -> Self {
        let mut result = Self {
            data: [0; GAMEBOY_MEMORY_SIZE_BYTES],
            cartridge: cartridge
        };

        // Interrupt Flags (IF) initial value
        result.data[0xFF0F] = 0xE0;

        result
    }

    /// Write a byte of data into memory handling special areas appropriately.
    ///
    /// # Examples
    ///
    /// Writes can be made to any 16-bit address:
    ///
    /// ```
    /// # extern crate gameboy;
    /// # use gameboy::cartridge::{Cartridge, RomOnly};
    /// # let cartridge: Box<Cartridge> = Box::new(RomOnly::new(vec![0; 0x10000]));
    /// # let mut memory = gameboy::memory::Memory::new(cartridge);
    /// memory.write_byte(0xCABC, 0x12);
    /// ```
    pub fn write_byte(&mut self, address: Address, value: u8) {
        match address {
            // Cartridge ROM
            0x0000..=0x7FFF => {
                // Ignore, cannot write to ROM
            },
            // Cartridge RAM (if available)
            0xA000..=0xBFFF => {
                self.cartridge.write_byte(address, value);
            },
            // WRAM (mirrored at 0xE000 - 0xFDFF)
            0xC000..=0xDDFF => {
                self.data[address as usize] = value;
                self.data[(address + 0x2000) as usize] = value;
            },
            // WRAM Mirror (mirrors R/W to 0xC000 - 0xDDFF)
            0xE000..=0xFDFF => {
                self.data[address as usize] = value;
                self.data[(address - 0x2000) as usize] = value;
            },
            // Unused RAM (0xFEA0 - 0xFEFF)
            0xFEA0..=0xFEFF => {
                // Do nothing, ignore writes here
                // TODO: Handle CGB mode weirdness
            },
            // Interrupt Flags (IF, 0xFF0F)
            0xFF0F => {
                // Only the lower 5-bits are R/W, the rest are always high. So
                // we clear the upper 3 bits and then set them high on write.
                self.data[address as usize] = (value & !0xE0) | 0xE0;
            },
            _ => {
                self.data[address as usize] = value
            }
        }
    }

    /// Read a byte of data from memory.
    ///
    /// # Examples
    ///
    /// Reads can be done from any 16-bit address:
    ///
    /// ```
    /// # extern crate gameboy;
    /// # use gameboy::cartridge::{Cartridge, RomOnly};
    /// # let cartridge: Box<Cartridge> = Box::new(RomOnly::new(vec![0; 0x10000]));
    /// # let mut memory = gameboy::memory::Memory::new(cartridge);
    /// memory.write_byte(0xCABC, 0x12);
    /// assert_eq!(memory.read_byte(0xCABC), 0x12);
    /// ```
    pub fn read_byte(&self, address: Address) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_byte(address),
            0xA000..=0xBFFF => self.cartridge.read_byte(address),
            _ => self.data[address as usize]
        }
    }

    pub fn read_word(&self, address: Address) -> u16 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_word(address),
            0xA000..=0xBFFF => self.cartridge.read_word(address),
            _ => {
                make_u16(self.read_byte(address), self.read_byte(address + 1))
            }
        }
    }
}
