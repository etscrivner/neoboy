extern crate gameboy;

use gameboy::*;
use gameboy::memory::Memory;
use gameboy::cpu::*;
use gameboy::operations::*;
use gameboy::rom::{CartridgeKind, Rom};

// Creates a new RomOnly cartridge from the given data.
//
// Array of bytes given can be of any size and will be copied to the execution
// start address (0x0100).
fn create_memory_from_cartridge_data(data: &[u8]) -> Memory {
    let mut cartridge_data = vec![0x00; 0x10000];
    cartridge_data[0x0100..(0x100 + data.len())].copy_from_slice(data);
    cartridge_data[0x0147] = CartridgeKind::RomOnly as u8;
    let rom = Rom::new(cartridge_data.to_vec()).unwrap();
    Memory::new(rom.into_cartridge().unwrap())
}

// Initializes memory from data fragment and loads operation from the start of
// the fragment.
fn operation_from_memory_fragment(data: &[u8]) -> GameboyResult<Operation> {
    let memory = create_memory_from_cartridge_data(data);
    Operation::from_memory(0x0100, &memory)
}

macro_rules! assert_op {
    ( $op:expr, $bytes:expr, $operation:ident ) => {
        let result = operation_from_memory_fragment($bytes).unwrap();
        assert_eq!(result.opcode, Opcode::$operation, "'{}' opcode mismatch: {:02X?}", $op, $bytes);
        assert_eq!(result.prefix, $bytes[0], "'{}' prefix mismatch: {:02X?}", $op, $bytes);
    };
    ( $op:expr, $bytes:expr, $operation:ident ( $( $arg:expr ),* ) ) => {
        let result = operation_from_memory_fragment($bytes).unwrap();
        assert_eq!(result.opcode, Opcode::$operation($($arg),*), "'{}' opcode mismatch: {:02X?}", $op, $bytes);
        assert_eq!(result.prefix, $bytes[0], "'{}' prefix mismatch: {:02X?}", $op, $bytes);
    };
}

macro_rules! assert_err {
    ( $bytes:expr ) => {
        let result = operation_from_memory_fragment($bytes);
        assert!(result.is_err());
    }
}

#[test]
fn test_misc() {
    assert_op!("NOP", &[0x00], Nop);
    assert_op!("RLCA", &[0x07], Rlca);
    assert_op!("RRCA", &[0x0F], Rrca);
    assert_op!("STOP", &[0x10], Stop);
    assert_op!("RLA", &[0x17], Rla);
    assert_op!("RRA", &[0x1F], Rra);
    assert_op!("DAA", &[0x27], Daa);
    assert_op!("CPL", &[0x2F], Cpl);
    assert_op!("SCF", &[0x36], Scf);
    assert_op!("CCF", &[0x3F], Ccf);
    assert_op!("DI", &[0xF3], Di);
    assert_op!("EI", &[0xFB], Ei);
}

#[test]
fn test_ld() {
    assert_op!("LD BC, d16", &[0x01, 0x12, 0x34], Ld16RegImm(Reg16::BC, 0x1234));
    assert_op!("LD DE, d16", &[0x11, 0x12, 0x34], Ld16RegImm(Reg16::DE, 0x1234));
    assert_op!("LD HL, d16", &[0x21, 0x12, 0x34], Ld16RegImm(Reg16::HL, 0x1234));
    assert_op!("LD SP, d16", &[0x31, 0x12, 0x34], Ld16RegImm(Reg16::SP, 0x1234));

    assert_op!("LD B, d8", &[0x06, 0x12], Ld8RegImm(Reg8::B, 0x12));
    assert_op!("LD C, d8", &[0x0E, 0x12], Ld8RegImm(Reg8::C, 0x12));
    assert_op!("LD D, d8", &[0x16, 0x12], Ld8RegImm(Reg8::D, 0x12));
    assert_op!("LD E, d8", &[0x1E, 0x12], Ld8RegImm(Reg8::E, 0x12));
    assert_op!("LD H, d8", &[0x26, 0x12], Ld8RegImm(Reg8::H, 0x12));
    assert_op!("LD L, d8", &[0x2E, 0x12], Ld8RegImm(Reg8::L, 0x12));
    assert_op!("LD A, d8", &[0x3E, 0x12], Ld8RegImm(Reg8::A, 0x12));

    assert_op!("LD A, (BC)", &[0x0A], Ld8AccMem(Reg16::BC));
    assert_op!("LD A, (DE)", &[0x1A], Ld8AccMem(Reg16::DE));
    assert_op!("LD A, (HL+)", &[0x2A], Ld8AccMem(Reg16::HL));
    assert_op!("LD A, (HL+)", &[0x3A], Ld8AccMem(Reg16::HL));

    assert_op!("LD B, B", &[0x40], Ld8RegReg(Reg8::B, Reg8::B));
    assert_op!("LD B, C", &[0x41], Ld8RegReg(Reg8::B, Reg8::C));
    assert_op!("LD B, D", &[0x42], Ld8RegReg(Reg8::B, Reg8::D));
    assert_op!("LD B, E", &[0x43], Ld8RegReg(Reg8::B, Reg8::E));
    assert_op!("LD B, H", &[0x44], Ld8RegReg(Reg8::B, Reg8::H));
    assert_op!("LD B, L", &[0x45], Ld8RegReg(Reg8::B, Reg8::L));
    assert_op!("LD B, L", &[0x47], Ld8RegReg(Reg8::B, Reg8::A));

    assert_op!("LD C, B", &[0x48], Ld8RegReg(Reg8::C, Reg8::B));
    assert_op!("LD C, C", &[0x49], Ld8RegReg(Reg8::C, Reg8::C));
    assert_op!("LD C, D", &[0x4A], Ld8RegReg(Reg8::C, Reg8::D));
    assert_op!("LD C, E", &[0x4B], Ld8RegReg(Reg8::C, Reg8::E));
    assert_op!("LD C, H", &[0x4C], Ld8RegReg(Reg8::C, Reg8::H));
    assert_op!("LD C, L", &[0x4D], Ld8RegReg(Reg8::C, Reg8::L));
    assert_op!("LD C, L", &[0x4F], Ld8RegReg(Reg8::C, Reg8::A));

    assert_op!("LD D, B", &[0x50], Ld8RegReg(Reg8::D, Reg8::B));
    assert_op!("LD D, C", &[0x51], Ld8RegReg(Reg8::D, Reg8::C));
    assert_op!("LD D, D", &[0x52], Ld8RegReg(Reg8::D, Reg8::D));
    assert_op!("LD D, E", &[0x53], Ld8RegReg(Reg8::D, Reg8::E));
    assert_op!("LD D, H", &[0x54], Ld8RegReg(Reg8::D, Reg8::H));
    assert_op!("LD D, L", &[0x55], Ld8RegReg(Reg8::D, Reg8::L));
    assert_op!("LD D, L", &[0x57], Ld8RegReg(Reg8::D, Reg8::A));

    assert_op!("LD E, B", &[0x58], Ld8RegReg(Reg8::E, Reg8::B));
    assert_op!("LD E, C", &[0x59], Ld8RegReg(Reg8::E, Reg8::C));
    assert_op!("LD E, D", &[0x5A], Ld8RegReg(Reg8::E, Reg8::D));
    assert_op!("LD E, E", &[0x5B], Ld8RegReg(Reg8::E, Reg8::E));
    assert_op!("LD E, H", &[0x5C], Ld8RegReg(Reg8::E, Reg8::H));
    assert_op!("LD E, L", &[0x5D], Ld8RegReg(Reg8::E, Reg8::L));
    assert_op!("LD E, L", &[0x5F], Ld8RegReg(Reg8::E, Reg8::A));

    assert_op!("LD H, B", &[0x60], Ld8RegReg(Reg8::H, Reg8::B));
    assert_op!("LD H, C", &[0x61], Ld8RegReg(Reg8::H, Reg8::C));
    assert_op!("LD H, D", &[0x62], Ld8RegReg(Reg8::H, Reg8::D));
    assert_op!("LD H, E", &[0x63], Ld8RegReg(Reg8::H, Reg8::E));
    assert_op!("LD H, H", &[0x64], Ld8RegReg(Reg8::H, Reg8::H));
    assert_op!("LD H, L", &[0x65], Ld8RegReg(Reg8::H, Reg8::L));
    assert_op!("LD H, L", &[0x67], Ld8RegReg(Reg8::H, Reg8::A));

    assert_op!("LD L, B", &[0x68], Ld8RegReg(Reg8::L, Reg8::B));
    assert_op!("LD L, C", &[0x69], Ld8RegReg(Reg8::L, Reg8::C));
    assert_op!("LD L, D", &[0x6A], Ld8RegReg(Reg8::L, Reg8::D));
    assert_op!("LD L, E", &[0x6B], Ld8RegReg(Reg8::L, Reg8::E));
    assert_op!("LD L, H", &[0x6C], Ld8RegReg(Reg8::L, Reg8::H));
    assert_op!("LD L, L", &[0x6D], Ld8RegReg(Reg8::L, Reg8::L));
    assert_op!("LD L, L", &[0x6F], Ld8RegReg(Reg8::L, Reg8::A));

    assert_op!("LD A, B", &[0x78], Ld8RegReg(Reg8::A, Reg8::B));
    assert_op!("LD A, C", &[0x79], Ld8RegReg(Reg8::A, Reg8::C));
    assert_op!("LD A, D", &[0x7A], Ld8RegReg(Reg8::A, Reg8::D));
    assert_op!("LD A, E", &[0x7B], Ld8RegReg(Reg8::A, Reg8::E));
    assert_op!("LD A, H", &[0x7C], Ld8RegReg(Reg8::A, Reg8::H));
    assert_op!("LD A, L", &[0x7D], Ld8RegReg(Reg8::A, Reg8::L));
    assert_op!("LD A, L", &[0x7F], Ld8RegReg(Reg8::A, Reg8::A));

    assert_op!("LD A, ($FF00 + a8)", &[0xF0, 0x12], LdhAccMem(0x12));
    assert_op!("LD A, ($FF00 + C)", &[0xF2], LdcAccMem);

    assert_op!("LD HL, SP + r8", &[0xF8, 0x01], LdHlSp(1));
    assert_op!("LD HL, SP + r8", &[0xF8, !0x01], LdHlSp(-2));
    assert_op!("LD SP, HL", &[0xF9], LdSpHl);
    assert_op!("LD A, (a16)", &[0xFA, 0x12, 0x34], Ld8AccMemImm(0x1234));
}

#[test]
fn test_st() {
    assert_op!("LD (BC), A", &[0x02], St8MemRegAcc(Reg16::BC));
    assert_op!("LD (DE), A", &[0x12], St8MemRegAcc(Reg16::DE));
    assert_op!("LD (HL+), A", &[0x22], St8MemRegAcc(Reg16::HL));
    assert_op!("LD (HL-), A", &[0x32], St8MemRegAcc(Reg16::HL));

    assert_op!("LD (a16), SP", &[0x08, 0x12, 0x34], St16MemSp(0x1234));
    assert_op!("LD ($FF00 + a8), A", &[0xE0, 0x12], LdhMemAcc(0x12));
    assert_op!("LD ($FF00 + C), A", &[0xE2], LdcMemAcc);
    assert_op!("LD (a16), A", &[0xEA, 0x12, 0x34], St8MemImmAcc(0x1234));
}

#[test]
fn test_inc() {
    assert_op!("INC BC", &[0x03], Inc16Reg(Reg16::BC));
    assert_op!("INC DE", &[0x13], Inc16Reg(Reg16::DE));
    assert_op!("INC HL", &[0x23], Inc16Reg(Reg16::HL));
    assert_op!("INC SP", &[0x33], Inc16Reg(Reg16::SP));

    assert_op!("INC B", &[0x04], Inc8Reg(Reg8::B));
    assert_op!("INC C", &[0x0C], Inc8Reg(Reg8::C));
    assert_op!("INC D", &[0x14], Inc8Reg(Reg8::D));
    assert_op!("INC E", &[0x1C], Inc8Reg(Reg8::E));
    assert_op!("INC H", &[0x24], Inc8Reg(Reg8::H));
    assert_op!("INC L", &[0x2C], Inc8Reg(Reg8::L));
    assert_op!("INC A", &[0x3C], Inc8Reg(Reg8::A));
}

#[test]
fn test_dec() {
    assert_op!("INC BC", &[0x0B], Dec16Reg(Reg16::BC));
    assert_op!("INC DE", &[0x1B], Dec16Reg(Reg16::DE));
    assert_op!("INC DE", &[0x2B], Dec16Reg(Reg16::HL));
    assert_op!("INC DE", &[0x3B], Dec16Reg(Reg16::SP));

    assert_op!("DEC B", &[0x05], Dec8Reg(Reg8::B));
    assert_op!("DEC C", &[0x0D], Dec8Reg(Reg8::C));
    assert_op!("DEC D", &[0x15], Dec8Reg(Reg8::D));
    assert_op!("DEC E", &[0x1D], Dec8Reg(Reg8::E));
    assert_op!("DEC H", &[0x25], Dec8Reg(Reg8::H));
    assert_op!("DEC L", &[0x2D], Dec8Reg(Reg8::L));
    assert_op!("DEC A", &[0x3D], Dec8Reg(Reg8::A));
}

#[test]
fn test_arithmetic() {
    assert_op!("ADD HL, BC", &[0x09], Add16HlReg(Reg16::BC));
    assert_op!("ADD HL, DE", &[0x19], Add16HlReg(Reg16::DE));
    assert_op!("ADD HL, HL", &[0x29], Add16HlReg(Reg16::HL));
    assert_op!("ADD HL, SP", &[0x39], Add16HlReg(Reg16::SP));
    assert_op!("ADD SP, r8", &[0xE8, 0x01], AddSp(1));
    assert_op!("ADD SP, r8", &[0xE8, !0x01], AddSp(-2));

    assert_op!("ADD A, B", &[0x80], Add8Reg(Reg8::B));
    assert_op!("ADD A, C", &[0x81], Add8Reg(Reg8::C));
    assert_op!("ADD A, D", &[0x82], Add8Reg(Reg8::D));
    assert_op!("ADD A, E", &[0x83], Add8Reg(Reg8::E));
    assert_op!("ADD A, H", &[0x84], Add8Reg(Reg8::H));
    assert_op!("ADD A, L", &[0x85], Add8Reg(Reg8::L));
    assert_op!("ADD A, (HL)", &[0x86], Add8AccHl);
    assert_op!("ADD A, A", &[0x87], Add8Reg(Reg8::A));
    assert_op!("ADD A, d8", &[0xC6, 0x12], Add8Imm(0x12));

    assert_op!("ADC A, B", &[0x88], Adc8Reg(Reg8::B));
    assert_op!("ADC A, C", &[0x89], Adc8Reg(Reg8::C));
    assert_op!("ADC A, D", &[0x8A], Adc8Reg(Reg8::D));
    assert_op!("ADC A, E", &[0x8B], Adc8Reg(Reg8::E));
    assert_op!("ADC A, H", &[0x8C], Adc8Reg(Reg8::H));
    assert_op!("ADC A, L", &[0x8D], Adc8Reg(Reg8::L));
    assert_op!("ADC A, (HL)", &[0x8E], Adc8AccHl);
    assert_op!("ADC A, A", &[0x8F], Adc8Reg(Reg8::A));
    assert_op!("ADC A, d8", &[0xCE, 0x12], Adc8Imm(0x12));

    assert_op!("SUB A, B", &[0x90], Sub8Reg(Reg8::B));
    assert_op!("SUB A, C", &[0x91], Sub8Reg(Reg8::C));
    assert_op!("SUB A, D", &[0x92], Sub8Reg(Reg8::D));
    assert_op!("SUB A, E", &[0x93], Sub8Reg(Reg8::E));
    assert_op!("SUB A, H", &[0x94], Sub8Reg(Reg8::H));
    assert_op!("SUB A, L", &[0x95], Sub8Reg(Reg8::L));
    assert_op!("SUB A, (HL)", &[0x96], Sub8AccHl);
    assert_op!("SUB A, A", &[0x97], Sub8Reg(Reg8::A));
    assert_op!("SUB A, d8", &[0xD6, 0x12], Sub8Imm(0x12));

    assert_op!("SBC A, B", &[0x98], Sbc8Reg(Reg8::B));
    assert_op!("SBC A, C", &[0x99], Sbc8Reg(Reg8::C));
    assert_op!("SBC A, D", &[0x9A], Sbc8Reg(Reg8::D));
    assert_op!("SBC A, E", &[0x9B], Sbc8Reg(Reg8::E));
    assert_op!("SBC A, H", &[0x9C], Sbc8Reg(Reg8::H));
    assert_op!("SBC A, L", &[0x9D], Sbc8Reg(Reg8::L));
    assert_op!("SBC A, (HL)", &[0x9E], Sbc8AccHl);
    assert_op!("SBC A, A", &[0x9F], Sbc8Reg(Reg8::A));
    assert_op!("SBC A, d8", &[0xDE, 0x12], Sbc8Imm(0x12));
}

#[test]
fn test_bitwise_arithmetic() {
    assert_op!("AND A, B", &[0xA0], And8Reg(Reg8::B));
    assert_op!("AND A, C", &[0xA1], And8Reg(Reg8::C));
    assert_op!("AND A, D", &[0xA2], And8Reg(Reg8::D));
    assert_op!("AND A, E", &[0xA3], And8Reg(Reg8::E));
    assert_op!("AND A, H", &[0xA4], And8Reg(Reg8::H));
    assert_op!("AND A, L", &[0xA5], And8Reg(Reg8::L));
    assert_op!("AND A, (HL)", &[0xA6], And8AccHl);
    assert_op!("AND A, A", &[0xA7], And8Reg(Reg8::A));
    assert_op!("AND A, d8", &[0xE6, 0x12], And8Imm(0x12));

    assert_op!("XOR A, B", &[0xA8], Xor8Reg(Reg8::B));
    assert_op!("XOR A, C", &[0xA9], Xor8Reg(Reg8::C));
    assert_op!("XOR A, D", &[0xAA], Xor8Reg(Reg8::D));
    assert_op!("XOR A, E", &[0xAB], Xor8Reg(Reg8::E));
    assert_op!("XOR A, H", &[0xAC], Xor8Reg(Reg8::H));
    assert_op!("XOR A, L", &[0xAD], Xor8Reg(Reg8::L));
    assert_op!("XOR A, (HL)", &[0xAE], Xor8AccHl);
    assert_op!("XOR A, A", &[0xAF], Xor8Reg(Reg8::A));
    assert_op!("XOR A, d8", &[0xEE, 0x12], Xor8Imm(0x12));

    assert_op!("OR A, B", &[0xB0], Or8Reg(Reg8::B));
    assert_op!("OR A, C", &[0xB1], Or8Reg(Reg8::C));
    assert_op!("OR A, D", &[0xB2], Or8Reg(Reg8::D));
    assert_op!("OR A, E", &[0xB3], Or8Reg(Reg8::E));
    assert_op!("OR A, H", &[0xB4], Or8Reg(Reg8::H));
    assert_op!("OR A, L", &[0xB5], Or8Reg(Reg8::L));
    assert_op!("OR A, (HL)", &[0xB6], Or8AccHl);
    assert_op!("OR A, A", &[0xB7], Or8Reg(Reg8::A));
    assert_op!("OR A, d8", &[0xF6, 0x12], Or8Imm(0x12));

    assert_op!("CP A, B", &[0xB8], Cp8Reg(Reg8::B));
    assert_op!("CP A, C", &[0xB9], Cp8Reg(Reg8::C));
    assert_op!("CP A, D", &[0xBA], Cp8Reg(Reg8::D));
    assert_op!("CP A, E", &[0xBB], Cp8Reg(Reg8::E));
    assert_op!("CP A, H", &[0xBC], Cp8Reg(Reg8::H));
    assert_op!("CP A, L", &[0xBD], Cp8Reg(Reg8::L));
    assert_op!("CP A, (HL)", &[0xBE], Cp8AccHl);
    assert_op!("CP A, A", &[0xBF], Cp8Reg(Reg8::A));
    assert_op!("CP A, d8", &[0xFE, 0x12], Cp8Imm(0x12));
}

#[test]
fn test_jump() {
    assert_op!("JR NZ, s8", &[0x20, 0x01], Jr(Condition::NZ, 1));
    assert_op!("JR NZ, s8", &[0x20, !0x01], Jr(Condition::NZ, -2));
    assert_op!("JR Z, s8", &[0x28, 0x01], Jr(Condition::Z, 1));
    assert_op!("JR Z, s8", &[0x28, !0x01], Jr(Condition::Z, -2));
    assert_op!("JR NC, s8", &[0x30, 0x01], Jr(Condition::NC, 1));
    assert_op!("JR NC, s8", &[0x30, !0x01], Jr(Condition::NC, -2));
    assert_op!("JR C, s8", &[0x38, 0x01], Jr(Condition::C, 1));
    assert_op!("JR C, s8", &[0x38, !0x01], Jr(Condition::C, -2));

    assert_op!("JP NZ, a16", &[0xC2, 0x12, 0x34], Jp(Condition::NZ, 0x1234));
    assert_op!("JP Z, a16", &[0xCA, 0x12, 0x34], Jp(Condition::Z, 0x1234));
    assert_op!("JP NC, a16", &[0xD2, 0x12, 0x34], Jp(Condition::NC, 0x1234));
    assert_op!("JP C, a16", &[0xDA, 0x12, 0x34], Jp(Condition::C, 0x1234));
    assert_op!("JP a16", &[0xC3, 0x12, 0x34], JpImm(0x1234));
    assert_op!("JP HL", &[0xE9], JpHl);
}

#[test]
fn test_call_ret() {
    assert_op!("RET NZ", &[0xC0], RetCond(Condition::NZ));
    assert_op!("RET Z", &[0xC8], RetCond(Condition::Z));
    assert_op!("RET NC", &[0xD0], RetCond(Condition::NC));
    assert_op!("RET C", &[0xD8], RetCond(Condition::C));
    assert_op!("RET", &[0xC9], Ret);
    assert_op!("RETI", &[0xD9], Reti);

    assert_op!("CALL NZ, a16", &[0xC4, 0x12, 0x34], CallCond(Condition::NZ, 0x1234));
    assert_op!("CALL Z, a16", &[0xCC, 0x12, 0x34], CallCond(Condition::Z, 0x1234));
    assert_op!("CALL NC, a16", &[0xD4, 0x12, 0x34], CallCond(Condition::NC, 0x1234));
    assert_op!("CALL C, a16", &[0xDC, 0x12, 0x34], CallCond(Condition::C, 0x1234));
    assert_op!("CALL a16", &[0xCD, 0x12, 0x34], Call(0x1234));
}

#[test]
fn test_stack() {
    assert_op!("POP BC", &[0xC1], Pop(Reg16::BC));
    assert_op!("POP DE", &[0xD1], Pop(Reg16::DE));
    assert_op!("POP HL", &[0xE1], Pop(Reg16::HL));
    assert_op!("POP AF", &[0xF1], Pop(Reg16::AF));

    assert_op!("PUSH BC", &[0xC5], Push(Reg16::BC));
    assert_op!("PUSH DE", &[0xD5], Push(Reg16::DE));
    assert_op!("PUSH HL", &[0xE5], Push(Reg16::HL));
    assert_op!("PUSH AF", &[0xF5], Push(Reg16::AF));
}

#[test]
fn test_rst() {
    assert_op!("RST $00", &[0xC7], Rst(0x00));
    assert_op!("RST $08", &[0xCF], Rst(0x08));
    assert_op!("RST $10", &[0xD7], Rst(0x10));
    assert_op!("RST $18", &[0xDF], Rst(0x18));
    assert_op!("RST $20", &[0xE7], Rst(0x20));
    assert_op!("RST $28", &[0xEF], Rst(0x28));
    assert_op!("RST $30", &[0xF7], Rst(0x30));
    assert_op!("RST $38", &[0xFF], Rst(0x38));
}

#[test]
fn test_invalid_opcodes() {
    assert_err!(&[0xD3]);
    assert_err!(&[0xDB]);
    assert_err!(&[0xDD]);
    assert_err!(&[0xE3]);
    assert_err!(&[0xE4]);
    assert_err!(&[0xEB]);
    assert_err!(&[0xEC]);
    assert_err!(&[0xED]);
    assert_err!(&[0xFC]);
    assert_err!(&[0xFD]);
}
