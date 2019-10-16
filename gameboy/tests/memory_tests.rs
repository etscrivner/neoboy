extern crate gameboy;
use gameboy::memory::{Memory};
use gameboy::cartridge::{Cartridge, RomOnly};

// Helper method that creates a new memory instance with ROM-only cartridge
fn new_memory() -> Memory {
    let cartridge: Box<Cartridge> = Box::new(RomOnly::new(vec![0x12; 0x10000]));
    Memory::new(cartridge)
}

#[test]
fn test_wram_mirror_readwrite() {
    let mut memory = new_memory();
    for address in 0xE000..=0xFDFF {
        memory.write_byte(address, 0xBC);
        assert_eq!(memory.read_byte(address), 0xBC);
        assert_eq!(memory.read_byte(address - 0x2000), 0xBC);
    }

    for address in 0xC000..=0xDDFF {
        memory.write_byte(address, 0xAB);
        assert_eq!(memory.read_byte(address), 0xAB);
        assert_eq!(memory.read_byte(address + 0x2000), 0xAB);
    }
}

#[test]
fn test_unused_ram_readwrite() {
    let mut memory = new_memory();

    // Should read constant 0
    for address in 0xFEA0..=0xFEFF {
        assert_eq!(memory.read_byte(address), 0);
        memory.write_byte(address, 0xBC);
        assert_eq!(memory.read_byte(address), 0);
    }
}

#[test]
fn test_interrupt_flag_readwrite() {
    let mut memory = new_memory();

    // Check initial value
    assert_eq!(memory.read_byte(0xFF0F), 0xE0);

    // Check that high 3 bits are never cleared
    memory.write_byte(0xFF0F, 0x0F);
    assert_eq!(memory.read_byte(0xFF0F), 0xEF);
    memory.write_byte(0xFF0F, 0x10);
    assert_eq!(memory.read_byte(0xFF0F), 0xF0);
}

#[test]
fn test_cartridge_rom_readwrite() {
    let mut memory = new_memory();

    // Cannot write to ROM. Value of 0x12 means reads delegated to Cartridge
    // interface.
    assert_eq!(memory.read_byte(0x0000), 0x12);
    memory.write_byte(0x0000, 0x34);
    assert_eq!(memory.read_byte(0x0000), 0x12);
}
