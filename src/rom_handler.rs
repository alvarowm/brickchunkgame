use std::fs::File;
use crate::param_reader;

pub fn load_rom_in_ram(rom_buffer: Vec<u8>) -> [u8; 4096] {
    let mut rom: [u8; 4096] = [0; 4096];
    for (position, byte) in rom_buffer.iter().enumerate() {
        rom[position] = *byte;
    }
    rom
}

pub fn read_rom_from_file(args: Vec<String>) -> Vec<u8> {
    let mut rom_location = param_reader::read_rom_path(args);

    if rom_location.is_empty() {
        rom_location = "./E23PlusMarkII96in1.bin".to_string();
    }
    match File::open(&rom_location) {
        Err(why) => panic!("Houve um erro abrindo {}: {}", rom_location, why),
        Ok(file) => file,
    };

    std::fs::read(rom_location).unwrap()
}
