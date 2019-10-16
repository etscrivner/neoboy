use super::*;

/// Generic interface for all gameboy cartridges.
pub trait Cartridge {
    fn read_byte(&self, address: Address) -> u8;
    fn write_byte(&mut self, address: Address, value: u8);
    fn read_word(&self, address: Address) -> u16;
    fn write_word(&self, address: Address, value: u16);
}

/// A cartridge which only contains ROM data and supports no other features.
pub struct RomOnly {
    pub data: Vec<u8>
}

impl RomOnly {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data: data }
    }
}

impl Cartridge for RomOnly {
    fn read_byte(&self, address: Address) -> u8 {
        match address {
            0x0000..=0x7FFF => self.data[address as usize],
            _ => panic!("Unsupported read from address {:04X}", address)
        }
    }

    fn write_byte(&mut self, _address: Address, _value: u8) {
        // Do nothing because we have no writable memory
    }

    fn read_word(&self, address: Address) -> u16 {
        make_u16(self.read_byte(address), self.read_byte(address + 1))
    }

    fn write_word(&self, _address: Address, _value: u16) {
        // Do nothing because we have no writable memory
    }
}
