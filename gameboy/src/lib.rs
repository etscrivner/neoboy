pub mod cartridge;
pub mod cpu;
pub mod memory;

use std::fs::File;
use std::io::Read;
use std::io;

/// Represents a 16-bit memory address
pub type Address = u16;
/// Represents an 8-bit register value
pub type Reg8 = u8;
/// Represents a 16-bit register value
pub type Reg16 = u16;
/// Represents a 3-bit unsigned integer constant
pub type Const3 = u8;
/// Represents an 8-bit constant value
pub type Const8 = u8;
/// Represents a 16-bit constant value
pub type Const16 = u16;
/// Represents an 8-bit offset value
pub type Offset = u8;

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
