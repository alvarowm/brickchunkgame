use std::process;
use std::sync::{Arc, Mutex};
use crate::instruction_decoder;
use bit::BitIndex;
use crossterm::event::{KeyEvent, KeyEventKind};
use crate::properties_reader::STATIC_CONFIG;

pub struct CPU {
    //PC usa somente 12 bits (4096 enderecos)
    //program Counter
    pub pc: u16,
    //Interrupt mode
    pub ei: bool,
    pub timer_flag: bool,
    pub stack_register: u16,
    //Registradores
    pub r0: u8,
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
    pub r4: u8,
    //Acumulador
    pub acc: u8,
    pub timer_counter: u8,
    pub carry_flag: bool,
    //pA
    pub pa0: bool,
    pub pa1: bool,
    pub pa2: bool,
    pub pa3: bool,

    pub pp0: bool,
    pub pp1: bool,
    pub pp2: bool,
    pub pp3: bool,

    pub ps0: bool,
    pub ps1: bool,
    pub ps2: bool,
    pub ps3: bool,

    pub pm0: bool,
    pub pm1: bool,
    pub pm2: bool,
    pub pm3: bool,

    pub timer_enabled: bool,

}

pub fn res(cpu: &mut CPU) {
    //println!("res");
    cpu.ei = false;
    cpu.pc = 0;
    cpu.timer_flag = false;
    cpu.timer_counter = 0;
    cpu.carry_flag = false;
    sound_off_01001010(cpu);
    sound_one_01001000(cpu);
    cpu.pa0 = true;
    cpu.pa1 = true;
    cpu.pa2 = true;
    cpu.pa3 = true;
}

pub fn exec_instruction(cpu: &mut CPU, ram: &mut [u8; 256], rom: [u8; 4096]) {
    if STATIC_CONFIG.lock().unwrap().get("debug").unwrap() == "true" {
        debug(cpu, rom);
    }

    if STATIC_CONFIG.lock().unwrap().get("debug_ram").unwrap() == "true" {
        debug_ram(ram);
    }

    if STATIC_CONFIG.lock().unwrap().get("debug_vram").unwrap() == "true" {
        debug_vram(ram);
    }
    instruction_decoder::execute_correct_instruction(cpu, ram, rom);
}

fn debug_ram(ram: &mut [u8; 256]) {
    for x in 1..256 {
        print!("{} ", x);
        if ram[x].bit(0) { print!("[]") } else { print!("x") };
        if ram[x].bit(1) { print!("[]") } else { print!("x") };
        if ram[x].bit(2) { print!("[]") } else { print!("x") };
        if ram[x].bit(3) { print!("[]") } else { print!("x") };
        if ram[x].bit(4) { print!("[]") } else { print!("x") };
        if ram[x].bit(5) { print!("[]") } else { print!("x") };
        if ram[x].bit(6) { print!("[]") } else { print!("x") };
        if ram[x].bit(7) { println!("[]") } else { println!("x") };
    }
}

fn debug_vram(ram: &mut [u8; 256]) {
    for x in 176..256 {
        print!("{} ", x);
        if ram[x].bit(0) { print!("[]") } else { print!("x") };
        if ram[x].bit(1) { print!("[]") } else { print!("x") };
        if ram[x].bit(2) { print!("[]") } else { print!("x") };
        if ram[x].bit(3) { print!("[]") } else { print!("x") };
        if ram[x].bit(4) { print!("[]") } else { print!("x") };
        if ram[x].bit(5) { print!("[]") } else { print!("x") };
        if ram[x].bit(6) { print!("[]") } else { print!("x") };
        if ram[x].bit(7) { println!("[]") } else { println!("x") };
    }
}

fn debug(cpu: &mut CPU, rom: [u8; 4096]) {
    let instruction = rom[cpu.pc as usize];
    let instruction_mais_1 = rom[(cpu.pc + 1) as usize];

    print!("{:08b}", instruction);
    print!(" ");
    println!("{:08b}", instruction_mais_1);
    println!("cpu.timer_flag = {}", cpu.timer_flag);
    println!("cpu.timer_counter  = {}", cpu.timer_counter);
    println!("pc = {}", cpu.pc);
    println!("acc = {}", cpu.acc);
    println!("r0 = {}", cpu.r0);
    println!("r1 = {}", cpu.r1);
    println!("r2 = {}", cpu.r2);
    println!("r3 = {}", cpu.r3);
    println!("r4 = {}", cpu.r4);
    println!("pa0 = {}", cpu.pa0);
    println!("pa1 = {}", cpu.pa1);
    println!("pa2 = {}", cpu.pa2);
    println!("pa3 = {}", cpu.pa3);
    println!("pp0 = {}", cpu.pp0);
    println!("pp1 = {}", cpu.pp1);
    println!("pp2 = {}", cpu.pp2);
    println!("pp3 = {}", cpu.pp3);
    println!("ps0 = {}", cpu.ps0);
    println!("ps1 = {}", cpu.ps1);
    println!("ps2 = {}", cpu.ps2);
    println!("ps3 = {}", cpu.ps3);
    println!("pm0 = {}", cpu.pm0);
    println!("pm1 = {}", cpu.pm1);
    println!("pm2 = {}", cpu.pm2);
    println!("pm3 = {}", cpu.pm3);
    println!("im = {}", cpu.ei);
    println!("stack = {}", cpu.stack_register);
    println!("cf = {}", cpu.carry_flag);
    println!("timer_enabled = {}", cpu.timer_enabled);

    println!("----------------------------");
}

//ADC A,[R1R0]
pub fn adc_00001000(cpu: &mut CPU, ram: &[u8; 256]) {
    //println!("ADC A,[R1R0]");
    cpu.pc += 1;

    let resultado: u8 = cpu.acc + ram[((cpu.r1 << 4) + cpu.r0) as usize] + (if cpu.carry_flag { 1 } else { 0 });

    cpu.acc = 0;
    cpu.acc.set_bit(0, resultado.bit(0));
    cpu.acc.set_bit(1, resultado.bit(1));
    cpu.acc.set_bit(2, resultado.bit(2));
    cpu.acc.set_bit(3, resultado.bit(3));
    cpu.carry_flag = resultado.bit(4);
}

//ADD A,XH
pub fn add_010000000000dddd(cpu: &mut CPU, data: u8) {
    //println!("ADD A,XH");
    cpu.pc += 2;

    let resultado = cpu.acc + data;

    cpu.acc = 0;
    cpu.acc.set_bit(0, resultado.bit(0));
    cpu.acc.set_bit(1, resultado.bit(1));
    cpu.acc.set_bit(2, resultado.bit(2));
    cpu.acc.set_bit(3, resultado.bit(3));
    cpu.carry_flag = resultado.bit(4);
}

//ADD A,[R1R0]
pub fn add_00001001(cpu: &mut CPU, ram: &[u8; 256]) {
    //println!("ADD A,[R1R0]");
    cpu.pc += 1;

    let resultado = cpu.acc + ram[((cpu.r1 << 4) + cpu.r0) as usize];

    cpu.acc = 0;
    cpu.acc.set_bit(0, resultado.bit(0));
    cpu.acc.set_bit(1, resultado.bit(1));
    cpu.acc.set_bit(2, resultado.bit(2));
    cpu.acc.set_bit(3, resultado.bit(3));
    cpu.carry_flag = resultado.bit(4);
}

//AND A,XH
pub fn and_010000100000(cpu: &mut CPU, byte2: u8) {
    //println!("AND A,XH");
    cpu.pc += 2;
    cpu.acc &= byte2;
}

//AND A,[R1R0]
pub fn and_00011010(cpu: &mut CPU, ram: &[u8; 256]) {
    //println!("AND A,[R1R0]");
    cpu.pc += 1;

    let data = ram[((cpu.r1 << 4) + cpu.r0) as usize];
    cpu.acc &= data;
}


//AND [R1R0],A
pub fn and_00011101(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("AND [R1R0],A");
    cpu.pc += 1;

    ram[((cpu.r1 << 4) + cpu.r0) as usize] &= cpu.acc;
}

//CALL address
pub fn call_1111(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("CALL address");
    cpu.stack_register = cpu.pc + 2;

    cpu.pc.set_bit(0, byte2.bit(0));
    cpu.pc.set_bit(1, byte2.bit(1));
    cpu.pc.set_bit(2, byte2.bit(2));
    cpu.pc.set_bit(3, byte2.bit(3));
    cpu.pc.set_bit(4, byte2.bit(4));
    cpu.pc.set_bit(5, byte2.bit(5));
    cpu.pc.set_bit(6, byte2.bit(6));
    cpu.pc.set_bit(7, byte2.bit(7));
    cpu.pc.set_bit(8, byte1.bit(0));
    cpu.pc.set_bit(9, byte1.bit(1));
    cpu.pc.set_bit(10, byte1.bit(2));
    cpu.pc.set_bit(11, byte1.bit(3));
    cpu.ei = false;
}

//CLC
pub fn clc_00101010(cpu: &mut CPU) {
    //println!("CLC");
    cpu.pc += 1;
    cpu.carry_flag = false;
}

//DAA
pub fn daa_00110110(cpu: &mut CPU) {
    //println!("DAA");
    cpu.pc += 1;

    if cpu.acc > 9 || cpu.carry_flag {
        cpu.acc += 6;
        cpu.acc.set_bit(4, false);
        cpu.carry_flag = true;
    }
}

//DEC A
pub fn dec_00111111(cpu: &mut CPU) {
    //println!("DEC A");
    cpu.pc += 1;
    if cpu.acc == 0 {
        cpu.acc = 15
    } else {
        cpu.acc -= 1;
    }
}

//DEC Rn
pub fn dec_0001nnn1(cpu: &mut CPU, instruction: u8) {
    //println!("DEC Rn");
    cpu.pc += 1;

    let mut register: u8 = 0;
    register.set_bit(0, instruction.bit(1));
    register.set_bit(1, instruction.bit(2));
    register.set_bit(2, instruction.bit(3));

    match register {
        0 => { if cpu.r0 == 0 { cpu.r0 = 15 } else { cpu.r0 -= 1 } }
        1 => { if cpu.r1 == 0 { cpu.r1 = 15 } else { cpu.r1 -= 1 } }
        2 => { if cpu.r2 == 0 { cpu.r2 = 15 } else { cpu.r2 -= 1 } }
        3 => { if cpu.r3 == 0 { cpu.r3 = 15 } else { cpu.r3 -= 1 } }
        4 => { if cpu.r4 == 0 { cpu.r4 = 15 } else { cpu.r4 -= 1 } }
        _ => panic!("Registrador não faz parte da CPU.")
    }
}

//DEC [R1R0]
pub fn dec_00001101(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("DEC [R1R0]");
    cpu.pc += 1;

    if ram[((cpu.r1 << 4) + cpu.r0) as usize] == 0 {
        ram[((cpu.r1 << 4) + cpu.r0) as usize] = 15
    } else {
        ram[((cpu.r1 << 4) + cpu.r0) as usize] -= 1;
    }
}

//DEC [R3R2]
pub fn dec_00001111(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("DEC [R3R2]");
    cpu.pc += 1;

    if ram[((cpu.r3 << 4) + cpu.r2) as usize] == 0 {
        ram[((cpu.r3 << 4) + cpu.r2) as usize] = 15
    } else {
        ram[((cpu.r3 << 4) + cpu.r2) as usize] -= 1;
    }
}

//DI
pub fn di_00101101(cpu: &mut CPU) {
    //println!("DI");
    cpu.pc += 1;
    cpu.ei = false;
}

//EI
pub fn ei_00101100(cpu: &mut CPU) {
    //println!("EI");
    cpu.pc += 1;
    cpu.ei = true;
}

//HALT
pub fn halt_0011011100111110() {
    process::exit(1);
}

//IN A,Pi
//pM 0 0 1 1 0 0 1 0
pub fn in_00110010(cpu: &mut CPU) {
    //println!("IN A,Pi");
    cpu.pc += 1;

    let mut pm: u8 = 0;
    pm.set_bit(0, cpu.pm0);
    pm.set_bit(1, cpu.pm1);
    pm.set_bit(2, cpu.pm2);
    pm.set_bit(3, cpu.pm3);
    cpu.acc = pm;
}

//IN A,Pi
//pS 0 0 1 1 0 0 1 1
pub fn in_00110011(cpu: &mut CPU) {
    //println!("IN A,Pi");
    cpu.pc += 1;
    let mut ps: u8 = 0;
    ps.set_bit(0, cpu.ps0);
    ps.set_bit(1, cpu.ps1);
    ps.set_bit(2, cpu.ps2);
    ps.set_bit(3, cpu.ps3);
    cpu.ps0 = true;
    cpu.ps1 = true;
    cpu.ps2 = true;
    cpu.ps3 = true;
    cpu.acc = ps;
}

//IN A,Pi
//pP 0 0 1 1 0 1 0 0
pub fn in_00110100(cpu: &mut CPU) {
    //println!("IN A,Pi");
    cpu.pc += 1;
    let mut pp: u8 = 0;
    pp.set_bit(0, cpu.pp0);
    pp.set_bit(1, cpu.pp1);
    pp.set_bit(2, cpu.pp2);
    pp.set_bit(3, cpu.pp3);
    cpu.acc = pp;
    cpu.pp0 = true;
    cpu.pp1 = true;
    cpu.pp2 = true;
    cpu.pp3 = true;
}

//INC A
pub fn inc_00110001(cpu: &mut CPU) {
    //println!("INC A");
    cpu.pc += 1;
    if cpu.acc == 15 {
        cpu.acc = 0;
    } else {
        cpu.acc += 1;
    }
}

//INC Rn
pub fn inc_0001nnn0(cpu: &mut CPU, instruction: u8) {
    //println!("INC Rn");
    cpu.pc += 1;
    let mut register: u8 = 0;
    register.set_bit(0, instruction.bit(1));
    register.set_bit(1, instruction.bit(2));
    register.set_bit(2, instruction.bit(3));

    match register {
        0 => { if cpu.r0 == 15 { cpu.r0 = 0 } else { cpu.r0 += 1 } }
        1 => { if cpu.r1 == 15 { cpu.r1 = 0 } else { cpu.r1 += 1 } }
        2 => { if cpu.r2 == 15 { cpu.r2 = 0 } else { cpu.r2 += 1 } }
        3 => { if cpu.r3 == 15 { cpu.r3 = 0 } else { cpu.r3 += 1 } }
        4 => { if cpu.r4 == 15 { cpu.r4 = 0 } else { cpu.r4 += 1 } }
        _ => panic!("Registrador não faz parte da CPU.")
    }
}

//INC [R1R0]
pub fn inc_00001100(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("INC [R1R0]");
    cpu.pc += 1;
    if ram[((cpu.r1 << 4) + cpu.r0) as usize] == 15 {
        ram[((cpu.r1 << 4) + cpu.r0) as usize] = 0
    } else {
        ram[((cpu.r1 << 4) + cpu.r0) as usize] += 1;
    }
}

//INC [R3R2]
pub fn inc_00001110(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("INC [R3R2]");
    cpu.pc += 1;
    if ram[((cpu.r3 << 4) + cpu.r2) as usize] == 15 {
        ram[((cpu.r3 << 4) + cpu.r2) as usize] = 0
    } else {
        ram[((cpu.r3 << 4) + cpu.r2) as usize] += 1;
    }
}

//JAn address
pub fn ja_100nnaaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JAn address");
    let mut bit_do_acumulador: u8 = 0;
    bit_do_acumulador.set_bit(0, byte1.bit(3));
    bit_do_acumulador.set_bit(1, byte1.bit(4));

    if cpu.acc.bit(bit_do_acumulador as usize) {
        cpu.pc.set_bit(0, byte2.bit(0));
        cpu.pc.set_bit(1, byte2.bit(1));
        cpu.pc.set_bit(2, byte2.bit(2));
        cpu.pc.set_bit(3, byte2.bit(3));
        cpu.pc.set_bit(4, byte2.bit(4));
        cpu.pc.set_bit(5, byte2.bit(5));
        cpu.pc.set_bit(6, byte2.bit(6));
        cpu.pc.set_bit(7, byte2.bit(7));
        cpu.pc.set_bit(8, byte1.bit(0));
        cpu.pc.set_bit(9, byte1.bit(1));
        cpu.pc.set_bit(10, byte1.bit(2));
    } else {
        cpu.pc += 2;
    }
}

//JC address
pub fn jc_11000aaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JC address");
    if cpu.carry_flag {
        cpu.pc.set_bit(0, byte2.bit(0));
        cpu.pc.set_bit(1, byte2.bit(1));
        cpu.pc.set_bit(2, byte2.bit(2));
        cpu.pc.set_bit(3, byte2.bit(3));
        cpu.pc.set_bit(4, byte2.bit(4));
        cpu.pc.set_bit(5, byte2.bit(5));
        cpu.pc.set_bit(6, byte2.bit(6));
        cpu.pc.set_bit(7, byte2.bit(7));
        cpu.pc.set_bit(8, byte1.bit(0));
        cpu.pc.set_bit(9, byte1.bit(1));
        cpu.pc.set_bit(10, byte1.bit(2));
    } else {
        cpu.pc += 2;
    }
}

//JMP address
pub fn jmp_1110aaaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JMP address");
    cpu.pc.set_bit(0, byte2.bit(0));
    cpu.pc.set_bit(1, byte2.bit(1));
    cpu.pc.set_bit(2, byte2.bit(2));
    cpu.pc.set_bit(3, byte2.bit(3));
    cpu.pc.set_bit(4, byte2.bit(4));
    cpu.pc.set_bit(5, byte2.bit(5));
    cpu.pc.set_bit(6, byte2.bit(6));
    cpu.pc.set_bit(7, byte2.bit(7));
    cpu.pc.set_bit(8, byte1.bit(0));
    cpu.pc.set_bit(9, byte1.bit(1));
    cpu.pc.set_bit(10, byte1.bit(2));
    cpu.pc.set_bit(11, byte1.bit(3));
}

//JNC address
pub fn jnc_11001aaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JNC address");
    if !cpu.carry_flag {
        cpu.pc.set_bit(0, byte2.bit(0));
        cpu.pc.set_bit(1, byte2.bit(1));
        cpu.pc.set_bit(2, byte2.bit(2));
        cpu.pc.set_bit(3, byte2.bit(3));
        cpu.pc.set_bit(4, byte2.bit(4));
        cpu.pc.set_bit(5, byte2.bit(5));
        cpu.pc.set_bit(6, byte2.bit(6));
        cpu.pc.set_bit(7, byte2.bit(7));
        cpu.pc.set_bit(8, byte1.bit(0));
        cpu.pc.set_bit(9, byte1.bit(1));
        cpu.pc.set_bit(10, byte1.bit(2));
    } else {
        cpu.pc += 2;
    }
}

//JNZ A,address
pub fn jnz_10111aaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JNZ A,address");
    if cpu.acc != 0 {
        cpu.pc.set_bit(0, byte2.bit(0));
        cpu.pc.set_bit(1, byte2.bit(1));
        cpu.pc.set_bit(2, byte2.bit(2));
        cpu.pc.set_bit(3, byte2.bit(3));
        cpu.pc.set_bit(4, byte2.bit(4));
        cpu.pc.set_bit(5, byte2.bit(5));
        cpu.pc.set_bit(6, byte2.bit(6));
        cpu.pc.set_bit(7, byte2.bit(7));
        cpu.pc.set_bit(8, byte1.bit(0));
        cpu.pc.set_bit(9, byte1.bit(1));
        cpu.pc.set_bit(10, byte1.bit(2));
    } else {
        cpu.pc += 2;
    }
}

//JNZ Rn,address
pub fn jnz_rrrrraaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JNZ Rn,address");
    let mut num_registrador: u8 = 0;

    num_registrador.set_bit(3, byte1.bit(3));
    num_registrador.set_bit(4, byte1.bit(4));
    num_registrador.set_bit(5, byte1.bit(5));
    num_registrador.set_bit(6, byte1.bit(6));
    num_registrador.set_bit(7, byte1.bit(7));

    let mut if_jump = false;

    if num_registrador == 160 && cpu.r0 != 0 {
        if_jump = true;
    }

    if num_registrador == 168 && cpu.r1 != 0 {
        if_jump = true;
    }

    if num_registrador == 216 && cpu.r4 != 0 {
        if_jump = true;
    }

    if if_jump {
        cpu.pc.set_bit(0, byte2.bit(0));
        cpu.pc.set_bit(1, byte2.bit(1));
        cpu.pc.set_bit(2, byte2.bit(2));
        cpu.pc.set_bit(3, byte2.bit(3));
        cpu.pc.set_bit(4, byte2.bit(4));
        cpu.pc.set_bit(5, byte2.bit(5));
        cpu.pc.set_bit(6, byte2.bit(6));
        cpu.pc.set_bit(7, byte2.bit(7));
        cpu.pc.set_bit(8, byte1.bit(0));
        cpu.pc.set_bit(9, byte1.bit(1));
        cpu.pc.set_bit(10, byte1.bit(2));
    } else {
        cpu.pc += 2;
    }
}

//JTMR address
pub fn jtmr_11010aaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JTMR address");
    if cpu.timer_flag {
        cpu.pc.set_bit(0, byte2.bit(0));
        cpu.pc.set_bit(1, byte2.bit(1));
        cpu.pc.set_bit(2, byte2.bit(2));
        cpu.pc.set_bit(3, byte2.bit(3));
        cpu.pc.set_bit(4, byte2.bit(4));
        cpu.pc.set_bit(5, byte2.bit(5));
        cpu.pc.set_bit(6, byte2.bit(6));
        cpu.pc.set_bit(7, byte2.bit(7));
        cpu.pc.set_bit(8, byte1.bit(0));
        cpu.pc.set_bit(9, byte1.bit(1));
        cpu.pc.set_bit(10, byte1.bit(2));
    } else {
        cpu.pc += 2;
    }

    cpu.timer_flag = false;
}

//JZ A,address
pub fn jz_10110aaaaaaaaaaa(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("JZ A,address");
    if cpu.acc == 0 {
        cpu.pc.set_bit(0, byte2.bit(0));
        cpu.pc.set_bit(1, byte2.bit(1));
        cpu.pc.set_bit(2, byte2.bit(2));
        cpu.pc.set_bit(3, byte2.bit(3));
        cpu.pc.set_bit(4, byte2.bit(4));
        cpu.pc.set_bit(5, byte2.bit(5));
        cpu.pc.set_bit(6, byte2.bit(6));
        cpu.pc.set_bit(7, byte2.bit(7));
        cpu.pc.set_bit(8, byte1.bit(0));
        cpu.pc.set_bit(9, byte1.bit(1));
        cpu.pc.set_bit(10, byte1.bit(2));
    } else {
        cpu.pc += 2;
    }
}

//MOV A,Rn
pub fn mov_0010nnn1(cpu: &mut CPU, byte1: u8) {
    //println!("MOV A,Rn");
    cpu.pc += 1;

    let mut num_registrador: u8 = 0;

    num_registrador.set_bit(0, byte1.bit(1));
    num_registrador.set_bit(1, byte1.bit(2));
    num_registrador.set_bit(2, byte1.bit(3));

    match num_registrador {
        0 => cpu.acc = cpu.r0,
        1 => cpu.acc = cpu.r1,
        2 => cpu.acc = cpu.r2,
        3 => cpu.acc = cpu.r3,
        4 => cpu.acc = cpu.r4,
        _ => panic!("Não esse registrador!")
    }
}

//MOV A,TMRH
pub fn mov_00111011(cpu: &mut CPU) {
    //println!("MOV A,TMRH");
    cpu.pc += 1;
    cpu.acc = 0;
    cpu.acc.set_bit(0, cpu.timer_counter.bit(4));
    cpu.acc.set_bit(1, cpu.timer_counter.bit(5));
    cpu.acc.set_bit(2, cpu.timer_counter.bit(6));
    cpu.acc.set_bit(3, cpu.timer_counter.bit(7));
}

//MOV A,TMRL
pub fn mov_00111010(cpu: &mut CPU) {
    //println!("MOV A,TMRL");
    cpu.pc += 1;
    cpu.acc = 0;
    cpu.acc.set_bit(0, cpu.timer_counter.bit(0));
    cpu.acc.set_bit(1, cpu.timer_counter.bit(1));
    cpu.acc.set_bit(2, cpu.timer_counter.bit(2));
    cpu.acc.set_bit(3, cpu.timer_counter.bit(3));
}

//MOV A,XH
pub fn mov_0111dddd(cpu: &mut CPU, byte1: u8) {
    //println!("MOV A,XH");
    cpu.pc += 1;
    cpu.acc = 0;
    cpu.acc.set_bit(0, byte1.bit(0));
    cpu.acc.set_bit(1, byte1.bit(1));
    cpu.acc.set_bit(2, byte1.bit(2));
    cpu.acc.set_bit(3, byte1.bit(3));
}

//MOV A,[R1R0]
pub fn mov_00000100(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("MOV A,[R1R0]");
    cpu.pc += 1;
    cpu.acc = ram[((cpu.r1 << 4) + cpu.r0) as usize];
}

//MOV A,[R3R2]
pub fn mov_00000110(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("MOV A,[R3R2]");
    cpu.pc += 1;
    cpu.acc = ram[((cpu.r3 << 4) + cpu.r2) as usize];
}

//MOV R1R0,XXH
pub fn mov_0101dddd0000dddd(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("MOV R1R0,XXH");
    cpu.pc += 2;

    cpu.r0.set_bit(0, byte1.bit(0));
    cpu.r0.set_bit(1, byte1.bit(1));
    cpu.r0.set_bit(2, byte1.bit(2));
    cpu.r0.set_bit(3, byte1.bit(3));

    cpu.r1.set_bit(0, byte2.bit(0));
    cpu.r1.set_bit(1, byte2.bit(1));
    cpu.r1.set_bit(2, byte2.bit(2));
    cpu.r1.set_bit(3, byte2.bit(3));
}

//MOV R3R2,XXH
pub fn mov_0110dddd0000dddd(cpu: &mut CPU, byte1: u8, byte2: u8) {
    //println!("MOV R3R2,XXH");
    cpu.pc += 2;

    cpu.r2.set_bit(0, byte1.bit(0));
    cpu.r2.set_bit(1, byte1.bit(1));
    cpu.r2.set_bit(2, byte1.bit(2));
    cpu.r2.set_bit(3, byte1.bit(3));

    cpu.r3.set_bit(0, byte2.bit(0));
    cpu.r3.set_bit(1, byte2.bit(1));
    cpu.r3.set_bit(2, byte2.bit(2));
    cpu.r3.set_bit(3, byte2.bit(3));
}

//MOV r4,XH
pub fn mov_010001100000dddd(cpu: &mut CPU, byte2: u8) {
    //println!("MOV r4,XH");
    cpu.pc += 2;

    cpu.r4.set_bit(0, byte2.bit(0));
    cpu.r4.set_bit(1, byte2.bit(1));
    cpu.r4.set_bit(2, byte2.bit(2));
    cpu.r4.set_bit(3, byte2.bit(3));
}

//MOV Rn,A
pub fn mov_0010nnn0(cpu: &mut CPU, byte1: u8) {
    //println!("MOV Rn,A");
    cpu.pc += 1;

    let mut num_registrador: u8 = 0;

    num_registrador.set_bit(0, byte1.bit(1));
    num_registrador.set_bit(1, byte1.bit(2));
    num_registrador.set_bit(2, byte1.bit(3));

    if num_registrador == 0 {
        cpu.r0.set_bit(0, cpu.acc.bit(0));
        cpu.r0.set_bit(1, cpu.acc.bit(1));
        cpu.r0.set_bit(2, cpu.acc.bit(2));
        cpu.r0.set_bit(3, cpu.acc.bit(3));
    } else if num_registrador == 1 {
        cpu.r1.set_bit(0, cpu.acc.bit(0));
        cpu.r1.set_bit(1, cpu.acc.bit(1));
        cpu.r1.set_bit(2, cpu.acc.bit(2));
        cpu.r1.set_bit(3, cpu.acc.bit(3));
    } else if num_registrador == 2 {
        cpu.r2.set_bit(0, cpu.acc.bit(0));
        cpu.r2.set_bit(1, cpu.acc.bit(1));
        cpu.r2.set_bit(2, cpu.acc.bit(2));
        cpu.r2.set_bit(3, cpu.acc.bit(3));
    } else if num_registrador == 3 {
        cpu.r3.set_bit(0, cpu.acc.bit(0));
        cpu.r3.set_bit(1, cpu.acc.bit(1));
        cpu.r3.set_bit(2, cpu.acc.bit(2));
        cpu.r3.set_bit(3, cpu.acc.bit(3));
    } else {
        cpu.r4.set_bit(0, cpu.acc.bit(0));
        cpu.r4.set_bit(1, cpu.acc.bit(1));
        cpu.r4.set_bit(2, cpu.acc.bit(2));
        cpu.r4.set_bit(3, cpu.acc.bit(3));
    }
}

//MOV TMRH,A
pub fn mov_00111101(cpu: &mut CPU) {
    //println!("MOV TMRH,A");
    cpu.pc += 1;
    cpu.timer_counter.set_bit(4, cpu.acc.bit(0));
    cpu.timer_counter.set_bit(5, cpu.acc.bit(1));
    cpu.timer_counter.set_bit(6, cpu.acc.bit(2));
    cpu.timer_counter.set_bit(7, cpu.acc.bit(3));
}

//MOV TMRL,A
pub fn mov_00111100(cpu: &mut CPU) {
    //println!("MOV TMRL,A");
    cpu.pc += 1;
    cpu.timer_counter.set_bit(0, cpu.acc.bit(0));
    cpu.timer_counter.set_bit(1, cpu.acc.bit(1));
    cpu.timer_counter.set_bit(2, cpu.acc.bit(2));
    cpu.timer_counter.set_bit(3, cpu.acc.bit(3));
}

//MOV [R1R0],A
pub fn mov_00000101(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("MOV [R1R0],A");
    cpu.pc += 1;
    ram[((cpu.r1 << 4) + cpu.r0) as usize] = cpu.acc;
}

//MOV [R3R2],A
pub fn mov_00000111(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("MOV [R3R2],A");
    cpu.pc += 1;
    ram[((cpu.r3 << 4) + cpu.r2) as usize] = cpu.acc;
}

//NOP
pub fn nop_00111110(cpu: &mut CPU) {
    //println!("NOP");
    cpu.pc += 1;
}

//OR A,XH
pub fn or_010001000000dddd(cpu: &mut CPU, byte2: u8) {
    //println!("OR A,XH");
    cpu.pc += 2;
    let mut dado: u8 = 0;

    dado.set_bit(0, byte2.bit(0));
    dado.set_bit(1, byte2.bit(1));
    dado.set_bit(2, byte2.bit(2));
    dado.set_bit(3, byte2.bit(3));

    cpu.acc |= dado;
}

//OR A,[R1R0]
pub fn or_00011100(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("OR A,[R1R0]");
    cpu.pc += 1;

    cpu.acc |= ram[((cpu.r1 << 4) + cpu.r0) as usize];
}

//OR [R1R0],A
pub fn or_00011111(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("OR [R1R0],A");
    cpu.pc += 1;
    ram[((cpu.r1 << 4) + cpu.r0) as usize] |= cpu.acc;
}

//OUT PA,A
pub fn out_00110000(cpu: &mut CPU) {
    //println!("OUT PA,A");
    cpu.pc += 1;

    cpu.pa0 = cpu.acc.bit(0);
    cpu.pa1 = cpu.acc.bit(1);
    cpu.pa2 = cpu.acc.bit(2);
    cpu.pa3 = cpu.acc.bit(3);
}

//READ MR0A
pub fn read_01001110(cpu: &mut CPU, ram: &mut [u8; 256], rom: [u8; 4096]) {
    //println!("READ MR0A");
    let curent_page: u8 = (cpu.pc / 256) as u8;
    let mut rom_address: u16 = 0;
    cpu.pc += 1;

    rom_address.set_bit(0, cpu.r4.bit(0));
    rom_address.set_bit(1, cpu.r4.bit(1));
    rom_address.set_bit(2, cpu.r4.bit(2));
    rom_address.set_bit(3, cpu.r4.bit(3));

    rom_address.set_bit(4, cpu.acc.bit(0));
    rom_address.set_bit(5, cpu.acc.bit(1));
    rom_address.set_bit(6, cpu.acc.bit(2));
    rom_address.set_bit(7, cpu.acc.bit(3));


    rom_address.set_bit(8, curent_page.bit(0));
    rom_address.set_bit(9, curent_page.bit(1));
    rom_address.set_bit(10, curent_page.bit(2));
    rom_address.set_bit(11, curent_page.bit(3));

    let rom_code: u8 = rom[rom_address as usize];

    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(0, rom_code.bit(4));
    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(1, rom_code.bit(5));
    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(2, rom_code.bit(6));
    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(3, rom_code.bit(7));

    cpu.acc.set_bit(0, rom_code.bit(0));
    cpu.acc.set_bit(1, rom_code.bit(1));
    cpu.acc.set_bit(2, rom_code.bit(2));
    cpu.acc.set_bit(3, rom_code.bit(3));
}

//READ R4A
pub fn read_01001100(cpu: &mut CPU, ram: &mut [u8; 256], rom: [u8; 4096]) {
    //println!("READ R4A");
    let curent_page: u8 = (cpu.pc / 256) as u8;
    let mut rom_address: u16 = 0;
    cpu.pc += 1;

    rom_address.set_bit(0, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(0));
    rom_address.set_bit(1, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(1));
    rom_address.set_bit(2, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(2));
    rom_address.set_bit(3, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(3));

    rom_address.set_bit(4, cpu.acc.bit(0));
    rom_address.set_bit(5, cpu.acc.bit(1));
    rom_address.set_bit(6, cpu.acc.bit(2));
    rom_address.set_bit(7, cpu.acc.bit(3));

    rom_address.set_bit(8, curent_page.bit(0));
    rom_address.set_bit(9, curent_page.bit(1));
    rom_address.set_bit(10, curent_page.bit(2));
    rom_address.set_bit(11, curent_page.bit(3));

    let rom_code: u8 = rom[rom_address as usize];

    cpu.r4.set_bit(0, rom_code.bit(4));
    cpu.r4.set_bit(1, rom_code.bit(5));
    cpu.r4.set_bit(2, rom_code.bit(6));
    cpu.r4.set_bit(3, rom_code.bit(7));

    cpu.acc.set_bit(0, rom_code.bit(0));
    cpu.acc.set_bit(1, rom_code.bit(1));
    cpu.acc.set_bit(2, rom_code.bit(2));
    cpu.acc.set_bit(3, rom_code.bit(3));
}

//READF MR0A
pub fn read_01001111(cpu: &mut CPU, ram: &mut [u8; 256], rom: [u8; 4096]) {
    //println!("READF MR0A");
    cpu.pc += 1;
    let mut rom_address: u16 = 0;

    rom_address.set_bit(0, cpu.r4.bit(0));
    rom_address.set_bit(1, cpu.r4.bit(1));
    rom_address.set_bit(2, cpu.r4.bit(2));
    rom_address.set_bit(3, cpu.r4.bit(3));

    rom_address.set_bit(4, cpu.acc.bit(0));
    rom_address.set_bit(5, cpu.acc.bit(1));
    rom_address.set_bit(6, cpu.acc.bit(2));
    rom_address.set_bit(7, cpu.acc.bit(3));

    rom_address.set_bit(8, true);
    rom_address.set_bit(9, true);
    rom_address.set_bit(10, true);
    rom_address.set_bit(11, true);

    let rom_code: u8 = rom[rom_address as usize];

    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(0, rom_code.bit(4));
    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(1, rom_code.bit(5));
    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(2, rom_code.bit(6));
    ram[((cpu.r1 << 4) + cpu.r0) as usize].set_bit(3, rom_code.bit(7));

    cpu.acc.set_bit(0, rom_code.bit(0));
    cpu.acc.set_bit(1, rom_code.bit(1));
    cpu.acc.set_bit(2, rom_code.bit(2));
    cpu.acc.set_bit(3, rom_code.bit(3));
}

//READF R4A
pub fn read_01001101(cpu: &mut CPU, ram: &mut [u8; 256], rom: [u8; 4096]) {
    //println!("READF R4A");
    cpu.pc += 1;
    let mut rom_address: u16 = 0;

    rom_address.set_bit(0, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(0));
    rom_address.set_bit(1, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(1));
    rom_address.set_bit(2, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(2));
    rom_address.set_bit(3, ram[((cpu.r1 << 4) + cpu.r0) as usize].bit(3));

    rom_address.set_bit(4, cpu.acc.bit(0));
    rom_address.set_bit(5, cpu.acc.bit(1));
    rom_address.set_bit(6, cpu.acc.bit(2));
    rom_address.set_bit(7, cpu.acc.bit(3));

    rom_address.set_bit(8, true);
    rom_address.set_bit(9, true);
    rom_address.set_bit(10, true);
    rom_address.set_bit(11, true);

    let rom_code: u8 = rom[rom_address as usize];

    cpu.r4.set_bit(0, rom_code.bit(4));
    cpu.r4.set_bit(1, rom_code.bit(5));
    cpu.r4.set_bit(2, rom_code.bit(6));
    cpu.r4.set_bit(3, rom_code.bit(7));

    cpu.acc.set_bit(0, rom_code.bit(0));
    cpu.acc.set_bit(1, rom_code.bit(1));
    cpu.acc.set_bit(2, rom_code.bit(2));
    cpu.acc.set_bit(3, rom_code.bit(3));
}

//RET
pub fn ret_00101110(cpu: &mut CPU) {
    //println!("RET");
    cpu.pc.set_bit(0, cpu.stack_register.bit(0));
    cpu.pc.set_bit(1, cpu.stack_register.bit(1));
    cpu.pc.set_bit(2, cpu.stack_register.bit(2));
    cpu.pc.set_bit(3, cpu.stack_register.bit(3));
    cpu.pc.set_bit(4, cpu.stack_register.bit(4));
    cpu.pc.set_bit(5, cpu.stack_register.bit(5));
    cpu.pc.set_bit(6, cpu.stack_register.bit(6));
    cpu.pc.set_bit(7, cpu.stack_register.bit(7));
    cpu.pc.set_bit(8, cpu.stack_register.bit(8));
    cpu.pc.set_bit(9, cpu.stack_register.bit(9));
    cpu.pc.set_bit(10, cpu.stack_register.bit(10));
    cpu.pc.set_bit(11, cpu.stack_register.bit(11));
    cpu.stack_register = 0;
}

//RETI
pub fn reti_00101111(cpu: &mut CPU) {
    //println!("RETI");
    cpu.pc.set_bit(0, cpu.stack_register.bit(0));
    cpu.pc.set_bit(1, cpu.stack_register.bit(1));
    cpu.pc.set_bit(2, cpu.stack_register.bit(2));
    cpu.pc.set_bit(3, cpu.stack_register.bit(3));
    cpu.pc.set_bit(4, cpu.stack_register.bit(4));
    cpu.pc.set_bit(5, cpu.stack_register.bit(5));
    cpu.pc.set_bit(6, cpu.stack_register.bit(6));
    cpu.pc.set_bit(7, cpu.stack_register.bit(7));
    cpu.pc.set_bit(8, cpu.stack_register.bit(8));
    cpu.pc.set_bit(9, cpu.stack_register.bit(9));
    cpu.pc.set_bit(10, cpu.stack_register.bit(10));
    cpu.pc.set_bit(11, cpu.stack_register.bit(11));
    cpu.carry_flag = cpu.pc.bit(12);
    cpu.stack_register = 0;
}

//RL A
pub fn rl_00000001(cpu: &mut CPU) {
    //println!("RL A");
    cpu.pc += 1;

    cpu.acc <<= 1;
    cpu.acc.set_bit(0, cpu.acc.bit(4));
    cpu.carry_flag = cpu.acc.bit(4);
    cpu.acc.set_bit(4, false);
}

//RLC A
pub fn rlc_00000011(cpu: &mut CPU) {
    //println!("RLC A");
    cpu.pc += 1;

    cpu.acc <<= 1;
    cpu.acc.set_bit(0, cpu.carry_flag);
    cpu.carry_flag = cpu.acc.bit(4);
    cpu.acc.set_bit(4, false);
}

//RR A
pub fn rr_00000000(cpu: &mut CPU) {
    //println!("RR A");
    cpu.pc += 1;

    let bit3 = cpu.acc.bit(0);
    cpu.acc >>= 1;
    cpu.acc.set_bit(7, false);
    cpu.acc.set_bit(3, bit3);
    cpu.carry_flag = bit3;
}

//RRC A
pub fn rrc_00000010(cpu: &mut CPU) {
    //println!("RRC A");
    cpu.pc += 1;

    let bit3 = cpu.acc.bit(0);
    cpu.acc >>= 1;
    cpu.acc.set_bit(7, false);
    cpu.acc.set_bit(3, cpu.carry_flag);
    cpu.carry_flag = bit3;
}

//SBC A,[R1R0]
pub fn sbc_00001010(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("SBC A,[R1R0]");
    cpu.pc += 1;

    cpu.acc = cpu.acc + !(ram[((cpu.r1 << 4) + cpu.r0) as usize]) - 240 + if cpu.carry_flag {1} else {0};

    cpu.carry_flag = cpu.acc.bit(4);

    cpu.acc.set_bit(4,false);
}

//SOUND A
pub fn sound_01001011(cpu: &mut CPU) {
    //println!("SOUND A");
    cpu.pc += 1;
}

//SOUND LOOP
pub fn sound_loop_01001001(cpu: &mut CPU) {
    //println!("SOUND LOOP");
    cpu.pc += 1;
}

//SOUND OFF
pub fn sound_off_01001010(cpu: &mut CPU) {
    //println!("SOUND OFF");
    cpu.pc += 1;
}

//SOUND ONE
pub fn sound_one_01001000(cpu: &mut CPU) {
    //println!("SOUND ONE");
    cpu.pc += 1;
}

//SOUND n
pub fn sound_010001010000nnnn(cpu: &mut CPU) {
    //println!("SOUND n");
    cpu.pc += 2;
}

//STC
pub fn stc_00101011(cpu: &mut CPU) {
    //println!("STC");
    cpu.pc += 1;
    cpu.carry_flag = true;
}

//SUB A,XH
pub fn sub_010000010000dddd(cpu: &mut CPU, byte1: u8) {
    //println!("SUB A,XH");
    cpu.pc += 2;

    cpu.acc = cpu.acc + !(byte1) - 240 + 1;

    cpu.carry_flag = cpu.acc.bit(4);
    cpu.acc.set_bit(4,false);

}

//SUB A,[R1R0]
pub fn sub_00001011(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("SUB A,[R1R0]");
    cpu.pc += 1;

    cpu.acc = cpu.acc + !(ram[((cpu.r1 << 4) + cpu.r0) as usize]) - 240 + 1;
    cpu.carry_flag = cpu.acc.bit(4);
    cpu.acc.set_bit(4,false);
}

//TIMER OFF
pub fn timer_off_00111001(cpu: &mut CPU) {
    //println!("TIMER OFF");
    cpu.pc += 1;
    cpu.timer_enabled = false;
}

//TIMER ON
pub fn timer_on_00111000(cpu: &mut CPU) {
    //println!("TIMER ON");
    cpu.pc += 1;
    cpu.timer_enabled = true;
}

//TIMER XXH
pub fn timer_01000111dddddddd(cpu: &mut CPU, byte2: u8) {
    //println!("TIMER XXH");
    cpu.pc += 2;
    cpu.timer_counter = byte2;
}

//XOR A,XH
pub fn xor_010000110000dddd(cpu: &mut CPU, byte2: u8) {
    //println!("XOR A,XH");
    cpu.pc += 2;
    cpu.acc ^= byte2;
}

//XOR A,[R1R0]
pub fn xor_00011011(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("XOR A,[R1R0]");
    cpu.pc += 1;

    cpu.acc ^= ram[((cpu.r1 << 4) + cpu.r0) as usize];
}

//XOR [R1R0],A
pub fn xor_00011110(cpu: &mut CPU, ram: &mut [u8; 256]) {
    //println!("XOR [R1R0],A");
    cpu.pc += 1;
    ram[((cpu.r1 << 4) + cpu.r0) as usize] ^= cpu.acc;
}

pub fn check_interrupts(cpu: &mut CPU, rom: [u8; 4096], mutex: &Arc<Mutex<Vec<KeyEvent>>>) {
    let instruction = rom[cpu.pc as usize];
    if mutex.lock().unwrap().len() > 0 && !(instruction.bit(0) &&
        instruction.bit(1) &&
        instruction.bit(2) &&
        instruction.bit(3)) {
        let key = mutex.lock().unwrap().pop().unwrap();

        if key.kind == KeyEventKind::Press {
            match key.code {
                //Start
                crossterm::event::KeyCode::Char('p') => {
                    cpu.ps0 = false;

                    if cpu.ei {
                        set_stack_cf(cpu);
                        cpu.pc = 8
                    }
                }
                //on/off
                crossterm::event::KeyCode::Char('o') => {
                    cpu.ps2 = false;


                    if cpu.ei {
                        set_stack_cf(cpu);
                        cpu.pc = 8
                    }
                }
                //Mute
                crossterm::event::KeyCode::Char('m') => {
                    cpu.ps1 = false;
                    if cpu.ei {
                        set_stack_cf(cpu);
                        cpu.pc = 8
                    }
                }
                //Mute/Left
                crossterm::event::KeyCode::Char('a') => {
                    cpu.pp3 = false;


                    if cpu.ei {
                        set_stack_cf(cpu);
                        cpu.pc = 8
                    }
                }
                //Select/Down
                crossterm::event::KeyCode::Char('s') => {
                    if cpu.ei {
                        set_stack_cf(cpu);
                        cpu.pc = 8
                    }

                    cpu.pp1 = false;
                }
                //Mute/Right
                crossterm::event::KeyCode::Char('d') => {
                    cpu.pp2 = false;

                    if cpu.ei {
                        set_stack_cf(cpu);
                        cpu.pc = 8
                    }
                }
                //Rotate
                crossterm::event::KeyCode::Char('r') => {
                    cpu.pp0 = false;

                    if cpu.ei {
                        set_stack_cf(cpu);
                        cpu.pc = 8
                    }
                }
                _ => {}
            }
        }
    }
}

fn set_stack_cf(cpu: &mut CPU) {
    cpu.stack_register = cpu.pc;
    cpu.stack_register.set_bit(12, cpu.carry_flag);
}
