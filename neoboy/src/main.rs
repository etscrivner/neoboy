extern crate gameboy;

use std::io;
use std::env;
use std::collections::HashMap;
use gameboy::*;

fn expected_value(regname: &str) -> u8 {
    match regname {
        "00" => 0x00,
        "08" => 0x08,
        "10" => 0x10,
        "18" => 0x18,
        "20" => 0x20,
        "28" => 0x28,
        "30" => 0x30,
        "38" => 0x38,
        "B" => 0,
        "C" => 1,
        "D" => 2,
        "E" => 3,
        "H" => 4,
        "L" => 5,
        "A" => 7,
        "BC" => 0,
        "DE" => 1,
        "HL" => 2,
        "HL+" => 2,
        "HL-" => 3,
        "AF" => 3,
        "SP" => 3,
        "NZ" => 0,
        "Z" => 1,
        "NC" => 2,
        "CC" => 3,
        _ => 0xFF
    }
}

fn opcode_tests() {
    let mut opcode_table: HashMap<&str, Vec<(u8, &str, Box<(Fn(u8) -> u8)>)>> = HashMap::new();
    let mut dualop_opcode_table: HashMap<
            &str, Vec<(u8, &str, &str, Box<(Fn(u8) -> u8)>, Box<(Fn(u8) -> u8)>)>> = HashMap::new();
    let mut covered_prefixes = vec![];
    let expected_gaps = vec![0xD3, 0xE3, 0xE4, 0xF4, 0xDB, 0xEB, 0xEC, 0xFC, 0xDD, 0xED, 0xFD];

    macro_rules! op {
        ( $name:expr, $prefix:expr ) => {
            covered_prefixes.push($prefix);
        };
        ( $name:expr, [ $( ($prefix:expr, $regl:expr, $regr:expr) ),* ], $convfnl:expr, $convfnr:expr ) => {
            let closurel = Box::new($convfnl);
            let closurer = Box::new($convfnr);
            $(
                covered_prefixes.push($prefix);
                dualop_opcode_table.entry(&$name).or_insert(vec![]).push(($prefix, $regl, $regr, closurel.clone(), closurer.clone()));
            )*
        };
        ( $name:expr, [ $( ($prefix:expr, $reg:expr) ),* ], $convfn:expr ) => {
            let closure = Box::new($convfn);
            $(
                covered_prefixes.push($prefix);
                opcode_table.entry(&$name).or_insert(vec![]).push(($prefix, $reg, closure.clone()));
            )*
        };
        
    }

    op!("NOP", 0x00);
    op!("RLCA", 0x07);
    op!("RLA", 0x17);
    op!("DAA", 0x27);
    op!("SCF", 0x37);
    op!("LD (a16), SP", 0x08);
    op!("JR s8", 0x18);
    op!("RRCA", 0x0F);
    op!("RRA", 0x1F);
    op!("CPL", 0x2F);
    op!("CCF", 0x3F);
    op!("STOP 0", 0x10);
    op!("INC (HL)", 0x34);
    op!("DEC (HL)", 0x35);
    op!("LD (HL), d8", 0x36);
    op!("HALT", 0x76);
    op!("ADD A, (HL)", 0x86);
    op!("ADC A, (HL)", 0x8E);
    op!("SUB (HL)", 0x96);
    op!("SBC A, (HL)", 0x9E);
    op!("AND (HL)", 0xA6);
    op!("XOR (HL)", 0xAE);
    op!("OR (HL)", 0xB6);
    op!("CP (HL)", 0xBE);
    op!("JP a16", 0xC3);
    op!("ADD A, d8", 0xC6);
    op!("SUB d8", 0xD6);
    op!("AND d8", 0xE6);
    op!("OR d8", 0xF6);
    op!("RET", 0xC9);
    op!("RETI", 0xD9);
    op!("PREFIX CB", 0xCB);
    op!("CALL a16", 0xCD);
    op!("ADC A, d8", 0xCE);
    op!("SBC A, d8", 0xDE);
    op!("XOR d8", 0xEE);
    op!("CP d8", 0xFE);
    op!("LDH ($FF00 + a8), A", 0xE0);
    op!("LDH A, ($FF00 + a8)", 0xF0);
    op!("LD (C), A", 0xE2);
    op!("LD A, (C)", 0xF2);
    op!("ADD SP, s8", 0xE8);
    op!("LD HL, SP+s8", 0xF8);
    op!("JP (HL)", 0xE9);
    op!("LD (a16), A", 0xEA);
    op!("LD A, (a16)", 0xFA);
    op!("DI", 0xF3);
    op!("LD SP, HL", 0xF9);
    op!("EI", 0xFB);
    op!(
        "LD r16, A",
        [(0x02, "BC"), (0x12, "DE"), (0x22, "HL+"), (0x32, "HL-")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "LD r8, d8",
        [(0x06, "B"), (0x0E, "C"), (0x16, "D"), (0x1E, "E"), (0x26, "H"), (0x2E, "L"), (0x3E, "A")],
        |value: u8| { (value >> 3) & 0x07 }
    );
    op!(
        "INC r16",
        [(0x03, "BC"), (0x13, "DE"), (0x23, "HL"), (0x33, "SP")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "INC r8",
        [(0x04, "B"), (0x0C, "C"), (0x14, "D"), (0x1C, "E"), (0x24, "H"), (0x2C, "L"), (0x3C, "A")],
        |value: u8| { (value >> 3) & 0x07 }
    );
    op!(
        "LD (HL), r8",
        [(0x70, "B"), (0x71, "C"), (0x72, "D"), (0x73, "E"), (0x74, "H"), (0x75, "L"), (0x77, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "LD r16, d16",
        [(0x01, "BC"), (0x11, "DE"), (0x21, "HL"), (0x31, "SP")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "DEC r8",
        [(0x05, "B"), (0x0D, "C"), (0x15, "D"), (0x1D, "E"), (0x25, "H"), (0x2D, "L"), (0x3D, "A")],
        |value: u8| { (value >> 3) & 0x07 }
    );
    op!(
        "DEC r16",
        [(0x0B, "BC"), (0x1B, "DE"), (0x2B, "HL"), (0x3B, "SP")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "ADD HL, r16",
        [(0x09, "BC"), (0x19, "DE"), (0x29, "HL"), (0x39, "SP")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "LD A, (r16)",
        [(0x0A, "BC"), (0x1A, "DE"), (0x2A, "HL+"), (0x3A, "HL-")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "JR cond, s8",
        [(0x20, "NZ"), (0x28, "Z"), (0x30, "NC"), (0x38, "CC")],
        |value: u8| { (value >> 3) & 0x03 }
    );
    op!(
        "LD r8, r8",
        [
            (0x40, "B", "B"), (0x41, "B", "C"), (0x42, "B", "D"), (0x43, "B", "E"), (0x44, "B", "H"), (0x45, "B", "L"), (0x47, "B", "A"),
            (0x48, "C", "B"), (0x49, "C", "C"), (0x4A, "C", "D"), (0x4B, "C", "E"), (0x4C, "C", "H"), (0x4D, "C", "L"), (0x4F, "C", "A"),
            (0x50, "D", "B"), (0x51, "D", "C"), (0x52, "D", "D"), (0x53, "D", "E"), (0x54, "D", "H"), (0x55, "D", "L"), (0x57, "D", "A"),
            (0x58, "E", "B"), (0x59, "E", "C"), (0x5A, "E", "D"), (0x5B, "E", "E"), (0x5C, "E", "H"), (0x5D, "E", "L"), (0x5F, "E", "A"),
            (0x60, "H", "B"), (0x61, "H", "C"), (0x62, "H", "D"), (0x63, "H", "E"), (0x64, "H", "H"), (0x65, "H", "L"), (0x67, "H", "A"),
            (0x68, "L", "B"), (0x69, "L", "C"), (0x6A, "L", "D"), (0x6B, "L", "E"), (0x6C, "L", "H"), (0x6D, "L", "L"), (0x6F, "L", "A"),
            (0x78, "A", "B"), (0x79, "A", "C"), (0x7A, "A", "D"), (0x7B, "A", "E"), (0x7C, "A", "H"), (0x7D, "A", "L"), (0x7F, "A", "A")
        ],
        |value: u8| { (value >> 3) & 0x07 },
        |value: u8| { value & 0x07 }
    );
    op!(
        "LD r8, (HL)",
        [(0x46, "B"), (0x4E, "C"), (0x56, "D"), (0x5E, "E"), (0x66, "H"), (0x6E, "L"), (0x7E, "A")],
        |value: u8| { (value >> 3) & 0x07 }
    );
    op!(
        "ADD A, r8",
        [(0x80, "B"), (0x81, "C"), (0x82, "D"), (0x83, "E"), (0x84, "H"), (0x85, "L"), (0x87, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "ADC A, r8",
        [(0x88, "B"), (0x89, "C"), (0x8A, "D"), (0x8B, "E"), (0x8C, "H"), (0x8D, "L"), (0x8F, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "SUB r8",
        [(0x90, "B"), (0x91, "C"), (0x92, "D"), (0x93, "E"), (0x94, "H"), (0x95, "L"), (0x97, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "SBC r8",
        [(0x98, "B"), (0x99, "C"), (0x9A, "D"), (0x9B, "E"), (0x9C, "H"), (0x9D, "L"), (0x9F, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "AND r8",
        [(0xA0, "B"), (0xA1, "C"), (0xA2, "D"), (0xA3, "E"), (0xA4, "H"), (0xA5, "L"), (0xA7, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "XOR r8",
        [(0xA8, "B"), (0xA9, "C"), (0xAA, "D"), (0xAB, "E"), (0xAC, "H"), (0xAD, "L"), (0xAF, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "OR r8",
        [(0xB0, "B"), (0xB1, "C"), (0xB2, "D"), (0xB3, "E"), (0xB4, "H"), (0xB5, "L"), (0xB7, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "CP r8",
        [(0xB8, "B"), (0xB9, "C"), (0xBA, "D"), (0xBB, "E"), (0xBC, "H"), (0xBD, "L"), (0xBF, "A")],
        |value: u8| { value & 0x07 }
    );
    op!(
        "RET cond",
        [(0xC0, "NZ"), (0xC8, "Z"), (0xD0, "NC"), (0xD8, "CC")],
        |value: u8| { (value >> 3) & 0x03 }
    );
    op!(
        "POP r16",
        [(0xC1, "BC"), (0xD1, "DE"), (0xE1, "HL"), (0xF1, "AF")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "JP cond, a16",
        [(0xC2, "NZ"), (0xCA, "Z"), (0xD2, "NC"), (0xDA, "CC")],
        |value: u8| { (value >> 3) & 0x03 }
    );
    op!(
        "CALL cond, a16",
        [(0xC4, "NZ"), (0xCC, "Z"), (0xD4, "NC"), (0xDC, "CC")],
        |value: u8| { (value >> 3) & 0x03 }
    );
    op!(
        "PUSH r16",
        [(0xC5, "BC"), (0xD5, "DE"), (0xE5, "HL"), (0xF5, "AF")],
        |value: u8| { (value >> 4) & 0x03 }
    );
    op!(
        "RST xx",
        [(0xC7, "00"), (0xCF, "08"), (0xD7, "10"), (0xDF, "18"), (0xE7, "20"), (0xEF, "28"), (0xF7, "30"), (0xFF, "38")],
        |value: u8| { value & 0x38 }
    );

    println!("TRANSLATION CHECK ({})", opcode_table.len() + dualop_opcode_table.len());
    for (name, entry) in opcode_table {
        print!("  {}", name);
        for (prefix, regname, conv_fn) in entry {
            if conv_fn(prefix) != expected_value(regname) {
                println!(
                    "\n  ERROR: {} - 0x{:02X} - {} is {} not {}",
                    name,
                    prefix,
                    regname,
                    conv_fn(prefix),
                    expected_value(regname)
                );
                return;
            }
        }
        println!(" - OK");
    }

    for (name, entry) in dualop_opcode_table {
        print!("  {}", name);
        for (prefix, regl, regr, conv_fnl, conv_fnr) in entry {
            if conv_fnl(prefix) != expected_value(regl) {
                println!(
                    "\n  ERROR: {} - 0x{:02X} - {} is {} not {}",
                    name,
                    prefix,
                    regl,
                    conv_fnl(prefix),
                    expected_value(regl)
                );
                return;
            }
            if conv_fnr(prefix) != expected_value(regr) {
                println!(
                    "\n  ERROR: {} - 0x{:02X} - {} is {} not {}",
                    name,
                    prefix,
                    regr,
                    conv_fnr(prefix),
                    expected_value(regr)
                );
                return;
            }
        }
        println!(" - OK");
    }

    print!("\nGAPS CHECK");
    for i in 0x00..=0xFF {
        if !covered_prefixes.contains(&i) && !expected_gaps.contains(&i) {
            println!("\n  ERROR: Missing prefix 0x{:02X}", i);
            return;
        }
    }
    println!(" - OK");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: neoboy [ROMFILE]");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, "Missing command-line arg"
        ));
    }

    let result = read_rom_file(&args[1])?;
    if let Ok(rom) = gameboy::rom::Rom::new(result) {
        println!("NAME: {:?}", rom.name());
        println!("KIND: {:?}", rom.kind());
        println!("VALID LOGO: {:?}", rom.has_valid_logo());
        println!("VALID HEADER CHECKSUM: {:?}", rom.has_valid_header_checksum());
        println!("VALID GLOBAL CHECKSUM: {:?}", rom.has_valid_global_checksum());
        if let Some(cartridge) = rom.into_cartridge() {
            let _memory = gameboy::memory::Memory::new(cartridge);
        } else {
            println!("error: unsupported ROM type.")
        }
    } else {
        println!("error: invalid cartridge");
    }

    Ok(())
}
