mod cpu;
mod utils;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use cpu::CPU;

use std::env;

fn main() -> io::Result<()> {
    let mut cpu = CPU::new();

    let args: Vec<String> = env::args().collect();

    // Prototyping reading from a ROM file
    //
    // iNES Format
    // Loading NROM cartridge
    // this logic should be specified in a mapper0 module
    //
    
    let mut file = File::open(args[1].to_string())?;

    let mut header = [0u8; 16];
    file.read(&mut header)?;

    // Check if the first three bytes are 'NES'
    assert_eq!(&header[..3], &[0x4E, 0x45, 0x53], "Not a valid iNES ROM!");

    // Write this to PPU 
    // Ignoring this since pattern tables aren't necessary until we implement CPU
    let prg_rom_size = header[4];

    // Write this to CPU
    let chr_rom_size = header[5];

    let mut prg_rom = vec![0; prg_rom_size as usize * 16384];
    file.read(&mut prg_rom)?;

    let mut chr_rom = vec![0; chr_rom_size as usize * 8192];
    file.read(&mut chr_rom)?;

    cpu.load_rom(&prg_rom);

    loop {
        cpu.emulate_cycle();
    }

    Ok(())
}
