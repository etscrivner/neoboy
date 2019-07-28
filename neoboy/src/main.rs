extern crate gameboy;

use std::io;
use std::env;
use gameboy::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: neoboy [ROMFILE]");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, "Missing command-line arg"
        ));
    }

    let result = read_rom_file(&args[1])?;
    if let Ok(cartridge) = gameboy::cartridge::Cartridge::new(result) {
        println!("NAME: {:?}", cartridge.name());
        println!("KIND: {:?}", cartridge.kind());
        println!("VALID LOGO: {:?}", cartridge.has_valid_logo());
        println!("VALID HEADER CHECKSUM: {:?}", cartridge.has_valid_header_checksum());
        println!("VALID GLOBAL CHECKSUM: {:?}", cartridge.has_valid_global_checksum());
    } else {
        println!("Error: Invalid Cartridge");
    }
    
    Ok(())
}
