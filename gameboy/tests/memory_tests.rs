extern crate gameboy;
use gameboy::memory::{Memory};

#[test]
fn test_memory_readwrite() {
    let mut memory = Memory::new();
    memory.write_byte(0x0000, 0xAF);
    assert_eq!(memory.read_byte(0x0000), 0xAF);
    memory.write_byte(0xFFFF, 0xBC);
    assert_eq!(memory.read_byte(0xFFFF), 0xBC);
    memory.write_byte(0x1234, 0xFF);
    assert_eq!(memory.read_byte(0x1234), 0xFF);
}

#[test]
fn test_wram_mirror_readwrite() {
    let mut memory = Memory::new();
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
    let mut memory = Memory::new();

    // Should read constant 0
    for address in 0xFEA0..=0xFEFF {
        assert_eq!(memory.read_byte(address), 0);
        memory.write_byte(address, 0xBC);
        assert_eq!(memory.read_byte(address), 0);
    }
}

#[test]
fn test_interrupt_flag_readwrite() {
    let mut memory = Memory::new();

    // Check initial value
    assert_eq!(memory.read_byte(0xFF0F), 0xE0);

    // Check that high 3 bits are never cleared
    memory.write_byte(0xFF0F, 0x0F);
    assert_eq!(memory.read_byte(0xFF0F), 0xEF);
    memory.write_byte(0xFF0F, 0x10);
    assert_eq!(memory.read_byte(0xFF0F), 0xF0);
}
