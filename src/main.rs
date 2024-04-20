use std::{env};
use std::sync::{Arc, Mutex};
use brickchunkgame::{param_reader, properties_reader, rom_handler, cpu, keyboard_reader, timer, lcd};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::event::KeyEvent;
use brickchunkgame::cpu::CPU;
use crate::properties_reader::STATIC_CONFIG;

fn main() {
    show_logo_version();

    let args: Vec<String> = env::args().collect();

    let rom_buffer = rom_handler::read_rom_from_file(args.to_owned());
    let config_file = param_reader::read_properties(args.to_owned());
    properties_reader::initialize_config(config_file);

    let rom: [u8; 4096] = rom_handler::load_rom_in_ram(rom_buffer);

    let mut cpu: CPU = CPU {
        pc: 0,
        ei: false,
        timer_flag: false,
        stack_register: 0,
        r0: 0,
        r1: 0,
        r2: 0,
        r3: 0,
        r4: 0,
        acc: 0,
        timer_counter: 1,
        carry_flag: false,
        pa0: false,
        pa1: false,
        pa2: false,
        pa3: false,
        pp0: true,
        pp1: true,
        pp2: true,
        pp3: true,
        ps0: true,
        ps1: true,
        ps2: true,
        ps3: true,
        pm0: true,
        pm1: true,
        pm2: true,
        pm3: true,
        timer_enabled: false,
    };

    let mut ram: [u8; 256] = [0; 256];

    let kb_mutex = Arc::new(Mutex::new(Vec::<KeyEvent>::new()));

    //start a thread to read the keyboard
    keyboard_reader::read_input(&kb_mutex);

    //(system clock)/2^4
    let timer_div:u8 = 16;
    let mut timer_counter:u8 = 0;

    //Para uso da lib de som do Holtek,
    //128KHz ou 64KHz sao aceitaveis
    let interval = Duration::from_micros(1);
    let mut next_time = Instant::now() + interval;

    //main loop
    loop {
        timer_counter +=1;
        if timer_counter == 3 {
            timer::tick(& mut cpu,rom);
            timer_counter = 0;
        }
        cpu::check_interrupts(& mut cpu, rom, &kb_mutex);
        cpu::exec_instruction(&mut cpu, &mut ram, rom);
        if STATIC_CONFIG.lock().unwrap().get("lcd").unwrap() == "true"{
            lcd::show(ram);
        }
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}

fn show_logo_version() {
    println!();
    //logo
    println!();
    println!("v{}", env!("CARGO_PKG_VERSION"));
    println!();
}
