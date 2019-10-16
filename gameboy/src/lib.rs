pub mod cartridge;
pub mod cpu;
pub mod machine;
pub mod memory;
pub mod operations;
pub mod registers;
pub mod rom;

use std::fs::File;
use std::io::Read;
use std::io;

/// Represents a 16-bit memory address
pub type Address = u16;
/// Represents a 3-bit unsigned integer constant
pub type Imm3 = u8;
/// Represents an 8-bit immediate value
pub type Imm8 = u8;
/// Represents a 16-bit immediate value
pub type Imm16 = u16;
/// Represents an 8-bit signed offset value
pub type Offset8 = i8;
/// Represents a cycle count
pub type Cycles = u16;

/// Generic result type for emulator errors
pub type GameboyResult<T> = std::result::Result<T, GameboyError>;

/// Emulator error type
#[derive(Debug, PartialEq)]
pub struct GameboyError {
    pub kind: GameboyErrorKind
}

/// Implementation of the error interface
impl GameboyError {
    /// Initialize a new error
    pub fn new(kind: GameboyErrorKind) -> Self {
        Self { kind: kind }
    }
}

/// A list of general categories of emulator errors
#[derive(Debug, PartialEq)]
pub enum GameboyErrorKind {
    /// Cartridge is too small to be valid. Contains size of cartridge data given.
    CartridgeTooSmall(usize),
    /// Attempt to load too much data into memory. Contains load start address and data size.
    MemoryLoadOutOfBounds(Address, usize),
    /// Opcode prefix was not recognized.
    UnknownOpcodePrefix(u8),
    /// Unknown ALU opcode prefix
    UnknownAluOpcodePrefix(u8),
    /// Unknown error with a description
    Unknown(String)
}

/// List of different support Game Boy types
pub enum GameboyType {
    /// DMG, this is the traditional monochrome gameboy
    DotMatrixGameboy,
    /// CGB, color gameboy
    ColorGameboy,
}

/// Top-level emulator configuration
pub struct Configuration {
    gameboy_type: GameboyType,
}

/// Read a rom file into a vector of bytes.
pub fn read_rom_file(rom_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(rom_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

/// Combines two 8-bit values into a single 16-bit value.
pub fn make_u16(msb: u8, lsb: u8) -> u16 {
    (msb as u16) << 8 | lsb as u16
}
