use super::*;
use super::memory::Memory;

/// 8-bit register constants
#[derive(Debug, PartialEq)]
pub enum Reg8 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    A = 7
}

/// 16-bit register constants
#[derive(Debug, PartialEq)]
pub enum Reg16 {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
    AF = 4
}

/// Enumeration of jump conditions
#[derive(Debug, PartialEq)]
pub enum Condition {
    /// Zero flag is set
    Z,
    /// Zero flag NOT set
    NZ,
    /// Carry flag is set
    C,
    /// Carry flag is NOT set
    NC
}

/// Enumeration of all operations for the Gameboy CPU.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    Adc8AccHl,
    Adc8Imm(Imm8),
    Adc8Reg(Reg8),
    Add16HlReg(Reg16),
    Add8AccHl,
    Add8Imm(Imm8),
    Add8Reg(Reg8),
    AddSp(Offset8),
    And8AccHl,
    And8Imm(Imm8),
    And8Reg(Reg8),
    Call(Imm16),
    CallCond(Condition, Imm16),
    Ccf,
    Cp8AccHl,
    Cp8Imm(Imm8),
    Cp8Reg(Reg8),
    Cpl,
    Daa,
    Dec16Reg(Reg16),
    Dec8MemHl,
    Dec8Reg(Reg8),
    Di,
    Ei,
    Inc16Reg(Reg16),
    Inc8MemHl,
    Inc8Reg(Reg8),
    Jp(Condition, Imm16),
    JpHl,
    JpImm(Imm16),
    Jr(Condition, Offset8),
    Ld16RegImm(Reg16, Imm16),
    Ld8AccMem(Reg16),
    Ld8AccMemImm(Imm16),
    St8MemImmAcc(Imm16),
    Ld8RegImm(Reg8, Imm8),
    Ld8RegMemHl(Reg8),
    Ld8RegReg(Reg8, Reg8),
    LdHlSp(Offset8),
    LdSpHl,
    LdcAccMem,
    LdcMemAcc,
    LdhAccMem(Imm8),
    LdhMemAcc(Imm8),
    Nop,
    Or8AccHl,
    Or8Imm(Imm8),
    Or8Reg(Reg8),
    Pop(Reg16),
    Push(Reg16),
    Ret,
    RetCond(Condition),
    Reti,
    Rla,
    Rlca,
    Rra,
    Rrca,
    Rst(Imm8),
    Sbc8AccHl,
    Sbc8Imm(Imm8),
    Sbc8Reg(Reg8),
    Scf,
    St16MemImmReg(Imm16, Reg16),
    St16MemSp(Imm16),
    St8MemRegAcc(Reg16),
    Stop,
    Sub8AccHl,
    Sub8Imm(Imm8),
    Sub8Reg(Reg8),
    Xor8AccHl,
    Xor8Imm(Imm8),
    Xor8Reg(Reg8)
 }

/// A single operation performed by the CPU.
#[derive(Debug, PartialEq)]
pub struct Operation {
    pub opcode: Opcode,
    pub prefix: u8
}

/// First method of converting a prefix into an 8-bit register.
fn prefix_into_reg8_1(prefix: u8) -> Reg8 {
    match (prefix >> 3) & 0x07 {
        0 => Reg8::B,
        1 => Reg8::C,
        2 => Reg8::D,
        3 => Reg8::E,
        4 => Reg8::H,
        5 => Reg8::L,
        _ => Reg8::A
    }
}

fn prefix_into_reg8_2(prefix: u8) -> Reg8 {
    match prefix & 0x07 {
        0 => Reg8::B,
        1 => Reg8::C,
        2 => Reg8::D,
        3 => Reg8::E,
        4 => Reg8::H,
        5 => Reg8::L,
        _ => Reg8::A
    }
}

/// First method of converting a prefix into a 16-bit register.
fn prefix_into_reg16_1(prefix: u8) -> Reg16 {
    match (prefix >> 4) & 0x03 {
        0 => Reg16::BC,
        1 => Reg16::DE,
        2 => Reg16::HL,
        _ => Reg16::SP
    }
}

/// Second method of converting a prefix into a 16-bit register for HL+/-
/// opcodes.
fn prefix_into_reg16_2(prefix: u8) -> Reg16 {
    match (prefix >> 4) & 0x03 {
        0 => Reg16::BC,
        1 => Reg16::DE,
        _ => Reg16::HL
    }
}

fn prefix_into_reg16_3(prefix: u8) -> Reg16 {
    match (prefix >> 4) & 0x03 {
        0 => Reg16::BC,
        1 => Reg16::DE,
        2 => Reg16::HL,
        _ => Reg16::AF
    }
}

fn prefix_into_cond(prefix: u8) -> Condition {
    match (prefix >> 3) & 0x03 {
        0 => Condition::NZ,
        1 => Condition::Z,
        2 => Condition::NC,
        _ => Condition::C
    }
}

impl Operation {
    /// Translate raw series of bytes into a CPU operation.
    pub fn from_memory(pc: Address, memory: &Memory) -> GameboyResult<Operation> {
        let prefix = memory.read_byte(pc);

        macro_rules! op {
            ( imm8 ) => {
                memory.read_byte(pc + 1)
            };
            ( imm16 ) => {
                memory.read_word(pc + 1)
            };
            ( s8 ) => {
                memory.read_byte(pc + 1) as Offset8
            };
            ( $opcode:ident ) => {
                Ok(Operation{ opcode: Opcode::$opcode, prefix: prefix })
            };
            ( $opcode:ident ( $arg:tt )) => {
                Ok(Operation{ opcode: Opcode::$opcode(op!($arg)), prefix: prefix })
            };
            ( $opcode:ident ( $arg:expr )) => {
                Ok(Operation{ opcode: Opcode::$opcode(op!($arg)), prefix: prefix })
            };
            ( $opcode:ident ( $argl:expr, $argr:tt )) => {
                Ok(Operation{ opcode: Opcode::$opcode(op!($argl), op!($argr)), prefix: prefix })
            };
            ( $opcode:ident ( $argl:tt, $argr:expr )) => {
                Ok(Operation{ opcode: Opcode::$opcode(op!($argl), op!($argr)), prefix: prefix })
            };
            ( $opcode:ident ( $argl:expr, $argr:expr )) => {
                Ok(Operation{ opcode: Opcode::$opcode(op!($argl), op!($argr)), prefix: prefix })
            };
            ( $ex:tt ) => {
                $ex
            };
        }

        match prefix {
            0x00 => op!(Nop),
            0x01 | 0x11 | 0x21 | 0x31 => op!(Ld16RegImm(prefix_into_reg16_1(prefix), imm16)),
            0x02 | 0x12 | 0x22 | 0x32 => op!(St8MemRegAcc(prefix_into_reg16_2(prefix))),
            0x03 | 0x13 | 0x23 | 0x33 => op!(Inc16Reg(prefix_into_reg16_1(prefix))),
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C => {
                op!(Inc8Reg(prefix_into_reg8_1(prefix)))
            },
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x3D => {
                op!(Dec8Reg(prefix_into_reg8_1(prefix)))
            },
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E => {
                op!(Ld8RegImm(prefix_into_reg8_1(prefix), imm8))
            },
            0x07 => op!(Rlca),
            0x08 => op!(St16MemSp(imm16)),
            0x09 | 0x19 | 0x29 | 0x39 => op!(Add16HlReg(prefix_into_reg16_1(prefix))),
            0x0A | 0x1A | 0x2A | 0x3A => op!(Ld8AccMem(prefix_into_reg16_2(prefix))),
            0x0B | 0x1B | 0x2B | 0x3B => op!(Dec16Reg(prefix_into_reg16_1(prefix))),
            0x0F => op!(Rrca),
            0x10 => op!(Stop),
            0x17 => op!(Rla),
            0x1F => op!(Rra),
            0x20 | 0x28 | 0x30 | 0x38 => op!(Jr(prefix_into_cond(prefix), s8)),
            0x27 => op!(Daa),
            0x2F => op!(Cpl),
            0x34 => op!(Inc8MemHl),
            0x35 => op!(Dec8MemHl),
            0x36 => op!(Scf),
            0x3F => op!(Ccf),
            0x40 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x47 |
            0x48 | 0x49 | 0x4A | 0x4B | 0x4C | 0x4D | 0x4F |
            0x50 | 0x51 | 0x52 | 0x53 | 0x54 | 0x55 | 0x57 |
            0x58 | 0x59 | 0x5A | 0x5B | 0x5C | 0x5D | 0x5F |
            0x60 | 0x61 | 0x62 | 0x63 | 0x64 | 0x65 | 0x67 |
            0x68 | 0x69 | 0x6A | 0x6B | 0x6C | 0x6D | 0x6F |
            0x78 | 0x79 | 0x7A | 0x7B | 0x7C | 0x7D | 0x7F => {
                op!(Ld8RegReg(prefix_into_reg8_1(prefix), prefix_into_reg8_2(prefix)))
            },
            0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x85 | 0x87 => {
                op!(Add8Reg(prefix_into_reg8_2(prefix)))
            },
            0x86 => op!(Add8AccHl),
            0x88 | 0x89 | 0x8A | 0x8B | 0x8C | 0x8D | 0x8F => {
                op!(Adc8Reg(prefix_into_reg8_2(prefix)))
            },
            0x8E => op!(Adc8AccHl),
            0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97 => {
                op!(Sub8Reg(prefix_into_reg8_2(prefix)))
            },
            0x96 => op!(Sub8AccHl),
            0x98 | 0x99 | 0x9A | 0x9B | 0x9C | 0x9D | 0x9F => {
                op!(Sbc8Reg(prefix_into_reg8_2(prefix)))
            },
            0x9E => op!(Sbc8AccHl),
            0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5 | 0xA7 => {
                op!(And8Reg(prefix_into_reg8_2(prefix)))
            },
            0xA6 => op!(And8AccHl),
            0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD | 0xAF => {
                op!(Xor8Reg(prefix_into_reg8_2(prefix)))
            },
            0xAE => op!(Xor8AccHl),
            0xB0 | 0xB1 | 0xB2 | 0xB3 | 0xB4 | 0xB5 | 0xB7 => {
                op!(Or8Reg(prefix_into_reg8_2(prefix)))
            },
            0xB6 => op!(Or8AccHl),
            0xB8 | 0xB9 | 0xBA | 0xBB | 0xBC | 0xBD | 0xBF => {
                op!(Cp8Reg(prefix_into_reg8_2(prefix)))
            },
            0xBE => op!(Cp8AccHl),
            0xC0 | 0xC8 | 0xD0 | 0xD8 => op!(RetCond(prefix_into_cond(prefix))),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => op!(Pop(prefix_into_reg16_3(prefix))),
            0xC2 | 0xCA | 0xD2 | 0xDA => op!(Jp(prefix_into_cond(prefix), imm16)),
            0xC3 => op!(JpImm(imm16)),
            0xC4 | 0xCC | 0xD4 | 0xDC => op!(CallCond(prefix_into_cond(prefix), imm16)),
            0xC5 | 0xD5 | 0xE5 | 0xF5 => op!(Push(prefix_into_reg16_3(prefix))),
            0xC6 => op!(Add8Imm(imm8)),
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                op!(Rst(prefix & 0x38))
            },
            0xC9 => op!(Ret),
            0xCB => Self::from_alu_prefix(memory.read_byte(pc + 1)), // CB Prefix
            0xCD => op!(Call(imm16)),
            0xCE => op!(Adc8Imm(imm8)),
            0xD6 => op!(Sub8Imm(imm8)),
            0xD9 => op!(Reti),
            0xDE => op!(Sbc8Imm(imm8)),
            0xE0 => op!(LdhMemAcc(imm8)),
            0xE2 => op!(LdcMemAcc),
            0xE6 => op!(And8Imm(imm8)),
            0xE8 => op!(AddSp(s8)),
            0xE9 => op!(JpHl),
            0xEA => op!(St8MemImmAcc(imm16)),
            0xEE => op!(Xor8Imm(imm8)),
            0xF0 => op!(LdhAccMem(imm8)),
            0xF2 => op!(LdcAccMem),
            0xF3 => op!(Di),
            0xF6 => op!(Or8Imm(imm8)),
            0xF8 => op!(LdHlSp(s8)),
            0xF9 => op!(LdSpHl),
            0xFA => op!(Ld8AccMemImm(imm16)),
            0xFB => op!(Ei),
            0xFE => op!(Cp8Imm(imm8)),
            _ => Err(
                GameboyError::new(GameboyErrorKind::UnknownOpcodePrefix(prefix))
            )
        }
    }

    // ALU operations starting with $CB prefix.
    //
    // In this method the $CB prefix is considered implied and the prefix
    // provided is the byte following the $CB prefix.
    fn from_alu_prefix(prefix: u8) -> GameboyResult<Operation> {
        Err(
            GameboyError::new(GameboyErrorKind::UnknownAluOpcodePrefix(prefix))
        )
    }
}

