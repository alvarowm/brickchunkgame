use std::{env};
use std::sync::{Arc, Mutex};
use brickchunkgame::{param_reader, properties_reader, rom_handler, cpu, keyboard_reader, timer, lcd};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::event::KeyEvent;
use brickchunkgame::cpu::CPU;
use crossterm::{
    terminal::{
        enable_raw_mode, EnterAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{Frame, prelude::{CrosstermBackend, Terminal}};
use std::io::{stdout, Result};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};
use brickchunkgame::lcd::LcdScreen;

static LCD_SCREEN: Mutex<LcdScreen> = Mutex::new(LcdScreen { rowsxcolumns: [[false; 10]; 20] });

fn main() -> Result<()> {
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
        timer_counter: 0,
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

    //ui
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    //Para uso da lib de som do Holtek,
    //128KHz ou 64KHz sao aceitaveis
    let interval = Duration::from_micros(1);
    let mut next_time = Instant::now() + interval;
    //main loop
    loop {
        timer::tick(&mut cpu, rom);

        cpu::check_interrupts(&mut cpu, rom, &kb_mutex);
        cpu::exec_instruction(&mut cpu, &mut ram, rom);

        LCD_SCREEN.lock().unwrap().rowsxcolumns = lcd::map_ram_to_lcd(ram).rowsxcolumns;

        terminal.draw(ui).unwrap();

        sleep(next_time - Instant::now());
        next_time += interval;
    }

    //stdout().execute(LeaveAlternateScreen)?;
    //disable_raw_mode()?;
    //Ok(())
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(100)
        ],
    )
        .split(frame.size());

    let buffer = frame.buffer_mut();

    for row in main_layout[0].rows() {
        for col in row.columns() {
            if col.x < 10 && col.y < 20 {
                if LCD_SCREEN.lock().unwrap().rowsxcolumns[col.y as usize][col.x as usize] {
                    let cell = buffer.get_mut(col.x, col.y);
                    cell.set_symbol("▮");
                } else {
                    let cell = buffer.get_mut(col.x, col.y);
                    cell.set_symbol("▯");
                }
            }
        }
    }

    frame.render_widget(
        Block::new().borders(Borders::NONE),
        main_layout[0],
    );
}


