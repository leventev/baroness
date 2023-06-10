use std::time::{SystemTime, UNIX_EPOCH};

use modular_bitfield::{bitfield, specifiers::B1};
use sdl2::{event::Event, render::Canvas, video::Window, EventPump};

use crate::{
    mapper::{get_mapper, Mapper},
    nes::NESFile,
};

use self::{cpu::CPUData, ppu::PPUData};

mod cpu;
mod ppu;

/// 2KiB internal memory
const INTERNAL_RAM_SIZE: usize = usize::pow(2, 11);

/// Original window width
pub const ORIGINAL_WIDTH: u32 = 256;

/// Original window height
pub const ORIGINAL_HEIGHT: u32 = 240;

/// How many times should the original resolution(256x240) be scaled up
pub const WINDOW_SCALE: u32 = 4;

/// Scaled window width
pub const WINDOW_WIDTH: u32 = WINDOW_SCALE * ORIGINAL_WIDTH;

/// Scaled window height
pub const WINDOW_HEIGHT: u32 = WINDOW_SCALE * ORIGINAL_HEIGHT;

#[bitfield]
#[derive(Clone)]
#[derive(Debug)]
pub struct StatusRegister {
    pub carry: B1,
    pub zero: B1,
    pub interrupt_disable: B1,
    pub decimal: B1,
    pub break_command: B1,
    pub always_set: B1,
    pub overflow: B1,
    pub negative: B1,
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub flags: StatusRegister,
}

pub struct Emulator {
    pub regs: Registers,
    internal_ram: Box<[u8]>,
    cpu: CPUData,
    ppu: PPUData,
    mapper: Box<dyn Mapper>,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    last_time: u128,
    frame_complete: bool,
    cycle_counter: usize,
}

impl Emulator {
    fn clock(&mut self) {
        self.clock_ppu();
        self.cycle_counter += 1;

        if self.cycle_counter == 3 {
            self.clock_cpu();
            self.cycle_counter = 0;
        }
    }

    fn emulate(&mut self) {
        const FRAME_TIME: u128 = 1_000_000_000 / 60;

        self.reset();

        let mut running = true;
        while running {
            println!("EVENT PUMP");
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => running = false,
                    _ => {}
                };
            }

            let mut new_frame = false;

            while !new_frame {
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();

                let elapsed = current_time - self.last_time;
                println!("{} {}", elapsed, FRAME_TIME);
                new_frame = elapsed > FRAME_TIME;
                if new_frame {
                    self.last_time = current_time;
                }
            }

            while !self.frame_complete {
                self.clock();
            }
            self.frame_complete = false;
        }
    }

    pub fn start_emulation(&mut self) {
        self.emulate();
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        if addr < 0x2000 {
            // internal ram
            let off = addr & 0x7FF;
            self.internal_ram[off as usize]
        } else if addr < 0x4000 {
            // ppu regs
            self.ppu_read_reg(addr as u8 % 8)
        } else if addr < 0x4020 {
            // apu, io registers
            //todo!()
            0
        } else {
            // cartridge space
            match self.mapper.read_cpu(addr) {
                Ok(val) => val,
                Err(_) => panic!("Open bus"),
            }
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            // internal ram
            let off = addr & 0x7FF;
            self.internal_ram[off as usize] = val;
        } else if addr < 0x4000 {
            // ppu regs
            self.ppu_write_reg(addr as u8 % 8, val);
        } else if addr < 0x4020 {
            // apu, io registers
            //todo!()
        } else {
            // cartridge space
            self.mapper.write_cpu(addr, val).unwrap();
        }
    }

    pub fn new(file: Vec<u8>, nes_file: NESFile) -> Emulator {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("baroness", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        let mapper = get_mapper(&file, &nes_file);
        Emulator {
            internal_ram: vec![0; INTERNAL_RAM_SIZE].into_boxed_slice(),
            regs: Registers {
                a: 0,
                x: 0,
                y: 0,
                sp: 0xFD,
                pc: mapper.entrypoint(),
                flags: StatusRegister::new().with_always_set(1),
            },
            cpu: CPUData::new(),
            ppu: PPUData::new(),
            mapper,
            canvas,
            event_pump,
            last_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            frame_complete: false,
            cycle_counter: 0,
        }
    }
}
