use super::{Address, GameboyError, GameboyErrorKind, GameboyResult};

// GameBoy contains 65,536 bytes of memory. While the whole space is
// addressable, many of the addresses in this space are unavailable for various
// reasons. The layout is as follows:
//
//     -- 0x0000 - 0x3FFF (ROM, Non-switchable ROM bank)
//     -- 0x4000 - 0x7FFF (ROMx, Switchable ROM bank)
//     -- 0x8000 - 0x9FFF (VRAM, switchable (0-1) in CGB)
//     -- 0xA000 - 0xBFFF (Cartridge RAM, often battery powered)
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
}

// TODO: Figure out how to switch between DMG and CGB behaviors
// TODO: Figure out how to handle I/O DMA addresses (callbacks?)
// TODO: Only store mirrored WRAM in one of the two locations?
impl Memory {
    /// Allocate new GameBoy main system memory and initializes various areas.
    pub fn new() -> Self {
        let mut result = Self {
            data: [0; GAMEBOY_MEMORY_SIZE_BYTES]
        };

        // Interrupt Flags (IF) initial value
        result.data[0xFF0F] = 0xE0;

        result
    }

    /// Attempts to load the given data into memory at the given address.
    ///
    /// This method performs basic bounds checking, but will allow data to be
    /// loaded at any location in memory including unwritable and DMA
    /// areas. Use with caution.
    ///
    /// # Examples
    ///
    /// Loading a slice into memory:
    ///
    /// ```
    /// # extern crate gameboy;
    /// # use gameboy::memory::Memory;
    /// # let mut memory = Memory::new();
    /// let bytes: [u8; 6] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
    /// assert!(memory.load(&bytes, 0x100, bytes.len()).is_ok());
    /// ```
    ///
    /// An error is returned if the slice runs out of bounds:
    ///
    /// ```
    /// # extern crate gameboy;
    /// # use gameboy::memory::Memory;
    /// # let mut memory = Memory::new();
    /// let bytes: [u8; 6] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
    /// assert!(memory.load(&bytes, 0xFFFA, bytes.len()).is_err());
    /// ```
    pub fn load(&mut self, data: &[u8], start_address: Address, size: usize) -> GameboyResult<()> {
        // Perform a check add on the range, if it overflows 16-bits then this
        // is too much data to be loaded.
        match start_address.checked_add(size as Address) {
            Some(_) => {
                let start = start_address as usize;
                let end = (start_address as usize) + size;
                self.data[start..end].copy_from_slice(data);
                Ok(())
            },
            _ => {
                return Err(GameboyError::new(
                    GameboyErrorKind::MemoryLoadOutOfBounds(start_address, size)
                ));
            }
        }
    }

    /// Write a byte of data into memory handling special areas appropriately.
    ///
    /// # Examples
    ///
    /// Writes can be made to any 16-bit address:
    ///
    /// ```
    /// # let mut memory = gameboy::memory::Memory::new();
    /// memory.write_byte(0xABCD, 0x12);
    /// ```
    pub fn write_byte(&mut self, address: Address, value: u8) {
        match address {
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
    /// # let mut memory = gameboy::memory::Memory::new();
    /// memory.write_byte(0xABCD, 0x12);
    /// assert_eq!(memory.read_byte(0xABCD), 0x12);
    /// ```
    pub fn read_byte(&self, address: Address) -> u8 {
        self.data[address as usize]
    }
}
