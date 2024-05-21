use bit::BitIndex;
use crate::cpu::CPU;

pub fn tick(cpu: &mut CPU, rom: [u8; 4096]){

    let instruction = rom[cpu.pc as usize];

    if cpu.ei && cpu.timer_flag &&
        !(instruction.bit(0) &&
            instruction.bit(1) &&
            instruction.bit(2) &&
            instruction.bit(3)) {
        cpu.stack_register = cpu.pc;
        let cf = cpu.carry_flag;
        cpu.stack_register.set_bit(12, cf);
        cpu.pc = 4;
        cpu.timer_flag = false;
    }

    if cpu.timer_enabled{
        if cpu.timer_counter == 255 {
            cpu.timer_flag = true;
            cpu.timer_counter = 0;
        } else {
            cpu.timer_counter +=1;
        }
    }
}