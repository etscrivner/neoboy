use super::{Address, GameboyError, GameboyErrorKind, GameboyResult};

use std::str;

/// Minimum size of a Gameboy cartridge (32 KB).
pub const MINIMUM_CARTRIDGE_SIZE_BYTES: usize = 0x8000;

/// Number of bytes in the Nintendo logo.
pub const NINTENDO_LOGO_SIZE_BYTES: usize = 48;

/// The Nintendo logo sprite data. Used to validate ROMs.
pub const NINTENDO_LOGO_BYTES: [u8; NINTENDO_LOGO_SIZE_BYTES] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// Represents a loaded Gameboy cartridge
pub struct Cartridge {
    pub data: Vec<u8>
}

/// Various types of cartridges as represented by the byte at ROM address 0x147
#[derive(Debug, PartialEq)]
pub enum CartridgeKind {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRam = 0x1D,
    Mbc5RumbleRamBattery = 0x1E,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF
}

impl Cartridge {
    /// Initialize a new cartridge from the given data.
    ///
    /// # Examples
    ///
    /// Creating a cartridge from a vector of data is simple:
    ///
    /// ```
    /// # use gameboy::cartridge::{Cartridge, MINIMUM_CARTRIDGE_SIZE_BYTES};
    /// let cartridge_data: [u8; MINIMUM_CARTRIDGE_SIZE_BYTES] = [0; MINIMUM_CARTRIDGE_SIZE_BYTES];
    /// assert!(Cartridge::new(cartridge_data.to_vec()).is_ok());
    /// ```
    ///
    /// Returns an error if the ROM is too small:
    ///
    /// ```
    /// # use gameboy::cartridge::Cartridge;
    /// let cartridge_data: [u8; 10] = [0; 10];
    /// assert!(Cartridge::new(cartridge_data.to_vec()).is_err());
    /// ```
    pub fn new(data: Vec<u8>) -> GameboyResult<Self> {
        if data.len() < MINIMUM_CARTRIDGE_SIZE_BYTES {
            return Err(GameboyError::new(
                GameboyErrorKind::CartridgeTooSmall(data.len())
            ));
        }

        Ok(Self { data: data })
    }

    pub fn kind(&self) -> Option<CartridgeKind> {
        match self.data[0x0147] {
            0x00 => Some(CartridgeKind::RomOnly),
            0x01 => Some(CartridgeKind::Mbc1),
            0x02 => Some(CartridgeKind::Mbc1Ram),
            0x03 => Some(CartridgeKind::Mbc1RamBattery),
            0x05 => Some(CartridgeKind::Mbc2),
            0x06 => Some(CartridgeKind::Mbc2Battery),
            0x08 => Some(CartridgeKind::RomRam),
            0x09 => Some(CartridgeKind::RomRamBattery),
            0x0B => Some(CartridgeKind::Mmm01),
            0x0C => Some(CartridgeKind::Mmm01Ram),
            0x0D => Some(CartridgeKind::Mmm01RamBattery),
            0x0F => Some(CartridgeKind::Mbc3TimerBattery),
            0x10 => Some(CartridgeKind::Mbc3TimerRamBattery),
            0x11 => Some(CartridgeKind::Mbc3),
            0x12 => Some(CartridgeKind::Mbc3Ram),
            0x13 => Some(CartridgeKind::Mbc3RamBattery),
            0x19 => Some(CartridgeKind::Mbc5),
            0x1A => Some(CartridgeKind::Mbc5Ram),
            0x1B => Some(CartridgeKind::Mbc5RamBattery),
            0x1C => Some(CartridgeKind::Mbc5Rumble),
            0x1D => Some(CartridgeKind::Mbc5RumbleRam), 
            0x1E => Some(CartridgeKind::Mbc5RumbleRamBattery),
            0x20 => Some(CartridgeKind::Mbc6),
            0x22 => Some(CartridgeKind::Mbc7SensorRumbleRamBattery),
            0xFC => Some(CartridgeKind::PocketCamera),
            0xFD => Some(CartridgeKind::BandaiTama5),
            0xFE => Some(CartridgeKind::HuC3),
            0xFF => Some(CartridgeKind::HuC1RamBattery),
            _ => None
        }
    }

    /// Indicates whether or not the cartridge contains a valid Nintendo logo.
    pub fn has_valid_logo(&self) -> bool {
        NINTENDO_LOGO_BYTES[..] == self.data[0x0104..=0x0133]
    }

    /// Indicates whether or not the ROM has a valid header checksum.
    ///
    /// The checksum is computed as follows:
    /// 
    ///   - Let checksum := 0x19
    ///   - For address in range [0x0134, 0x014D] (inclusive of both ends)
    ///     - checksum += ROM[address]
    ///   - Validate (checksum & 0xFF) == 0x00
    ///
    /// Essentially the byte at address 0x014D should overflow the final
    /// checksum back to zero for 8-bit wrapping addition.
    pub fn has_valid_header_checksum(&self) -> bool {
        let mut checksum: u8 = 0x19;
        for index in 0x0134..=0x014D {
            checksum = checksum.wrapping_add(self.data[index]);
        }
        checksum == 0x00
    }

    /// Indicates whether or not the ROM has a valid global checksum.
    ///
    /// The checksum is computed as follows (we skip summing checksum bytes):
    ///
    ///   - Let checksum := 0
    ///   - For address in range [0x0000, 0x014E]
    ///     - checksum += ROM[address]
    ///   - For address in range [0x0150, <ROM END>]
    ///     - checksum += ROM[address]
    ///   - Validate checksum == (ROM[0x014E] << 8) | ROM[0x014F]
    pub fn has_valid_global_checksum(&self) -> bool {
        let mut checksum: u16 = 0x00;
        let expected: u16 = (self.data[0x014E] as u16) << 8 | self.data[0x014F] as u16;

        for index in 0x0000..0x014E {
            checksum = checksum.wrapping_add(self.data[index] as u16);
        }

        for index in 0x0150..self.data.len() {
            checksum = checksum.wrapping_add(self.data[index] as u16);
        }

        checksum == expected
    }

    /// Name of the game stored on the cartridge.
    pub fn name(&self) -> String {
        let mut name: Vec<u8> = Vec::new();

        for ch in self.data[0x0134..=0x0143].iter() {
            if *ch == 0 {
                break;
            }

            name.push(*ch);
        }

        str::from_utf8(&name).unwrap().to_string()
    }

    /// Size of the cartridge in bytes.
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }
}
