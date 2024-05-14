use bit::BitIndex;
use crate::cpu::CPU;
use crate::cpu;

pub fn execute_correct_instruction(cpu: &mut CPU, ram: &mut [u8; 256], rom: [u8; 4096]){
    if cpu.pc.bit(12){
        println!("{}", cpu.pc);
        panic!("pc inválido");
    }

    if cpu.acc.bit(4){
        println!("{}", cpu.pc);
        panic!("acc inválido");
    }

    if cpu.stack_register.bit(13){
        panic!("sr inválido");
    }

    let instruction = rom[cpu.pc as usize];
    let instruction_mais_1 = rom[(cpu.pc + 1) as usize];

    let mut opcode_in_binary = "".to_string();

    opcode_in_binary += &format!("{:08b}", instruction);
    opcode_in_binary += &format!("{:08b}", instruction_mais_1);

    match opcode_in_binary {
        x if x.starts_with("00001000") => cpu::adc_00001000(cpu,ram),
        x if x.starts_with("010000000000") => cpu::add_010000000000dddd(cpu,instruction_mais_1),
        x if x.starts_with("00001001") => cpu::add_00001001(cpu,ram),
        x if x.starts_with("010000100000") => cpu::and_010000100000(cpu,instruction_mais_1),
        x if x.starts_with("00011010") => cpu::and_00011010(cpu,ram),
        x if x.starts_with("00011101") => cpu::and_00011101(cpu,ram),
        x if x.starts_with("1111") => cpu::call_1111(cpu,instruction, instruction_mais_1),
        x if x.starts_with("00101010") => cpu::clc_00101010(cpu),
        x if x.starts_with("00110110") => cpu::daa_00110110(cpu),
        x if x.starts_with("00111111") => cpu::dec_00111111(cpu),
        x if x.starts_with("00001101") => cpu::dec_00001101(cpu, ram),
        x if x.starts_with("00001111") => cpu::dec_00001111(cpu, ram),
        x if x.starts_with("00101101") => cpu::di_00101101(cpu),
        x if x.starts_with("00101100") => cpu::ei_00101100(cpu),
        x if x.starts_with("0011011100111110") => cpu::halt_0011011100111110(),
        x if x.starts_with("00110010") => cpu::in_00110010(cpu),
        x if x.starts_with("00110011") => cpu::in_00110011(cpu),
        x if x.starts_with("00110100") => cpu::in_00110100(cpu),
        x if x.starts_with("00110001") => cpu::inc_00110001(cpu),
        x if x.starts_with("00001100")  => cpu::inc_00001100(cpu,ram),
        x if x.starts_with("00001110")  => cpu::inc_00001110(cpu,ram),
        x if x.starts_with("100")  => cpu::ja_100nnaaaaaaaaaaa(cpu,instruction, instruction_mais_1),
        x if x.starts_with("11000")  => cpu::jc_11000aaaaaaaaaaa(cpu,instruction, instruction_mais_1),
        x if x.starts_with("1110")  => cpu::jmp_1110aaaaaaaaaaaa(cpu, instruction, instruction_mais_1),
        x if x.starts_with("11001")  => cpu::jnc_11001aaaaaaaaaaa(cpu, instruction, instruction_mais_1),
        x if x.starts_with("10111")  => cpu::jnz_10111aaaaaaaaaaa(cpu, instruction, instruction_mais_1),
        x if x.starts_with("10100") || x.starts_with("10101") || x.starts_with("11011") => cpu::jnz_rrrrraaaaaaaaaaa(cpu, instruction, instruction_mais_1),
        x if x.starts_with("11010")  => cpu::jtmr_11010aaaaaaaaaaa(cpu, instruction, instruction_mais_1),
        x if x.starts_with("10110")  => cpu::jz_10110aaaaaaaaaaa(cpu, instruction, instruction_mais_1),
        x if x.starts_with("00100001")  ||
            x.starts_with("00100011") ||
            x.starts_with("00100101") ||
            x.starts_with("00100111") ||
            x.starts_with("00101001")
        => cpu::mov_0010nnn1(cpu, instruction),
        x if x.starts_with("00111011")  => cpu::mov_00111011(cpu),
        x if x.starts_with("00111010")  => cpu::mov_00111010(cpu),
        x if x.starts_with("0111")  => cpu::mov_0111dddd(cpu, instruction),
        x if x.starts_with("00000100")  => cpu::mov_00000100(cpu, ram),
        x if x.starts_with("00000110")  => cpu::mov_00000110(cpu, ram),
        x if x.starts_with("0101")
            && x.chars().nth(8).unwrap() == '0'
            && x.chars().nth(9).unwrap() == '0'
            && x.chars().nth(10).unwrap() == '0'
            && x.chars().nth(11).unwrap() == '0' => cpu::mov_0101dddd0000dddd(cpu, instruction, instruction_mais_1),
        x if x.starts_with("0110")
            && x.chars().nth(8).unwrap() == '0'
            && x.chars().nth(9).unwrap() == '0'
            && x.chars().nth(10).unwrap() == '0'
            && x.chars().nth(11).unwrap() == '0' => cpu::mov_0110dddd0000dddd(cpu, instruction, instruction_mais_1),
        x if x.starts_with("010001100000") => cpu::mov_010001100000dddd(cpu, instruction_mais_1),

        x if x.starts_with("00111101")  => cpu::mov_00111101(cpu),
        x if x.starts_with("00111100")  => cpu::mov_00111100(cpu),
        x if x.starts_with("00000101")  => cpu::mov_00000101(cpu, ram),
        x if x.starts_with("00000111")  => cpu::mov_00000111(cpu, ram),
        x if x.starts_with("00111110")  => cpu::nop_00111110(cpu),
        x if x.starts_with("010001000000")  => cpu::or_010001000000dddd(cpu, instruction_mais_1),
        x if x.starts_with("00011100")  => cpu::or_00011100(cpu, ram),
        x if x.starts_with("00011111")  => cpu::or_00011111(cpu, ram),
        x if x.starts_with("00110000")  => cpu::out_00110000(cpu),
        x if x.starts_with("01001110")  => cpu::read_01001110(cpu,ram, rom),
        x if x.starts_with("01001100")  => cpu::read_01001100(cpu,ram, rom),
        x if x.starts_with("01001111")  => cpu::read_01001111(cpu,ram, rom),
        x if x.starts_with("01001101")  => cpu::read_01001101(cpu,ram, rom),
        x if x.starts_with("00101110")  => cpu::ret_00101110(cpu),
        x if x.starts_with("00101111")  => cpu::reti_00101111(cpu),
        x if x.starts_with("00000001")  => cpu::rl_00000001(cpu),
        x if x.starts_with("00000011")  => cpu::rlc_00000011(cpu),
        x if x.starts_with("00000000")  => cpu::rr_00000000(cpu),
        x if x.starts_with("00000010")  => cpu::rrc_00000010(cpu),
        x if x.starts_with("00001010")  => cpu::sbc_00001010(cpu, ram),
        x if x.starts_with("01001011")  => cpu::sound_01001011(cpu),
        x if x.starts_with("01001001")  => cpu::sound_loop_01001001(cpu),
        x if x.starts_with("01001010")  => cpu::sound_off_01001010(cpu),
        x if x.starts_with("01001000")  => cpu::sound_one_01001000(cpu),
        x if x.starts_with("010001010000")  => cpu::sound_010001010000nnnn(cpu),
        x if x.starts_with("00101011")  => cpu::stc_00101011(cpu),
        x if x.starts_with("010000010000")  => cpu::sub_010000010000dddd(cpu, instruction_mais_1),
        x if x.starts_with("00001011")  => cpu::sub_00001011(cpu, ram),
        x if x.starts_with("00111001")  => cpu::timer_off_00111001(cpu),
        x if x.starts_with("00111000")  => cpu::timer_on_00111000(cpu),
        x if x.starts_with("01000111")  => cpu::timer_01000111dddddddd(cpu, instruction_mais_1),
        x if x.starts_with("010000110000")  => cpu::xor_010000110000dddd(cpu, instruction_mais_1),
        x if x.starts_with("00011011")  => cpu::xor_00011011(cpu,ram),
        x if x.starts_with("00011110")  => cpu::xor_00011110(cpu,ram),
        x if x.starts_with("0001") && x.chars().nth(7).unwrap() == '1' => cpu::dec_0001nnn1(cpu, instruction),
        x if x.starts_with("0001") && x.chars().nth(7).unwrap() == '0' => cpu::inc_0001nnn0(cpu, instruction),
        x if x.starts_with("0010") && x.chars().nth(7).unwrap() == '0' => cpu::mov_0010nnn0(cpu, instruction),
        _ => panic!("Opcode não pertence ao set de instruções do HT1130")
    }


}