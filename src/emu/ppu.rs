use std::ops::Not;

use modular_bitfield::{
    bitfield,
    specifiers::{B1, B3, B5},
};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use super::{Emulator, WINDOW_SCALE};

const PPUCTRL: u8 = 0;
const PPUMASK: u8 = 1;
const PPUSTATUS: u8 = 2;
const OAMADDR: u8 = 3;
const OAMDATA: u8 = 4;
const PPUSCROLL: u8 = 5;
const PPUADDR: u8 = 6;
const PPUDATA: u8 = 7;

const PALETTE: &[u8] = &[
    84, 84, 84, 0, 30, 116, 8, 16, 144, 48, 0, 136, 68, 0, 100, 92, 0, 48, 84, 4, 0, 60, 24, 0, 32,
    42, 0, 8, 58, 0, 0, 64, 0, 0, 60, 0, 0, 50, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 152, 150, 152, 8,
    76, 196, 48, 50, 236, 92, 30, 228, 136, 20, 176, 160, 20, 100, 152, 34, 32, 120, 60, 0, 84, 90,
    0, 40, 114, 0, 8, 124, 0, 0, 118, 40, 0, 102, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 236, 238, 236,
    76, 154, 236, 120, 124, 236, 176, 98, 236, 228, 84, 236, 236, 88, 180, 236, 106, 100, 212, 136,
    32, 160, 170, 0, 116, 196, 0, 76, 208, 32, 56, 204, 108, 56, 180, 204, 60, 60, 60, 0, 0, 0, 0,
    0, 0, 236, 238, 236, 168, 204, 236, 188, 188, 236, 212, 178, 236, 236, 174, 236, 236, 174, 212,
    236, 180, 176, 228, 196, 144, 204, 210, 120, 180, 222, 120, 168, 226, 144, 152, 226, 180, 160,
    214, 228, 160, 162, 160, 0, 0, 0, 0, 0, 0,
];

#[bitfield]
#[derive(Debug, Clone)]
struct VRAMAddress {
    coarse_x: B5,
    coarse_y: B5,
    nametable_x: B1,
    nametable_y: B1,
    fine_y: B3,
    unused: B1,
}

#[bitfield]
#[derive(Debug)]
struct ControlReg {
    /// Nametable X
    nametable_x: B1,

    /// Nametable Y
    nametable_y: B1,

    /// 0 - VRAM address is incremented by 1, 1 - VRAM address is incremented by 32
    vram_increment: B1,

    /// 0 - 0x0000, 1 - 0x1000
    sprite_pattern_table_address: B1,

    /// 0 - 0x0000, 1 - 0x1000
    background_pattern_table_address: B1,

    /// 0 - 8x8, 1 - 8x16
    sprite_size: B1,

    ///
    master_slave_select: B1,

    /// Generate an NMI at the start of vertical blanking
    generate_nmi: B1,
}

#[bitfield]
struct MaskReg {
    greyscale: B1,
    show_background_leftmost: B1,
    show_sprites_leftmost: B1,
    show_background: B1,
    show_sprites: B1,
    emphasize_red: B1,
    emphasize_green: B1,
    emphasize_blue: B1,
}

pub struct PPUData {
    even_frame: bool,
    cycle: usize,
    scanline: usize,
    vertical_blanking: bool,
    nametables: [[u8; 1024]; 4],
    control_reg: ControlReg,
    mask_reg: MaskReg,
    second_byte: bool,
    vram_address: VRAMAddress,
    temp_vram_address: VRAMAddress,
    fine_x: u8,
    data_buffer: u8,
    lsb_shift_reg: u8,
    msb_shift_reg: u8,
    palette_table: [u8; 32],
    current_palette: u8,
    current_sprite_index: u8,
}

impl PPUData {
    pub fn new() -> PPUData {
        PPUData {
            even_frame: false,
            cycle: 0,
            scanline: 0,
            vertical_blanking: false,
            nametables: [[0; 1024]; 4],
            control_reg: ControlReg::new(),
            mask_reg: MaskReg::new(),
            second_byte: false,
            vram_address: VRAMAddress::new(),
            temp_vram_address: VRAMAddress::new(),
            fine_x: 0,
            data_buffer: 0,
            lsb_shift_reg: 0,
            msb_shift_reg: 0,
            palette_table: [0; 32],
            current_palette: 0,
            current_sprite_index: 0,
        }
    }
}

impl Emulator {
    pub fn ppu_read_reg(&mut self, reg: u8) -> u8 {
        assert!(reg < 8);
        match reg {
            PPUCTRL | PPUMASK | OAMADDR | PPUSCROLL | PPUADDR => 0,
            PPUSTATUS => {
                let mut res = 0;
                if self.ppu.vertical_blanking {
                    res |= 1 << 7;
                    self.ppu.vertical_blanking = false;
                }

                self.ppu.second_byte = false;

                res
            }
            PPUDATA => {
                assert!(!self.ppu.second_byte);

                let mut temp_addr = u16::from_ne_bytes(self.ppu.vram_address.bytes);
                let val = self.ppu_read(temp_addr);

                temp_addr += [1, 32][self.ppu.control_reg.vram_increment() as usize];
                self.ppu.vram_address = VRAMAddress::from_bytes(temp_addr.to_ne_bytes());

                let ret = if temp_addr > 0x3f00 {
                    val
                } else {
                    self.ppu.data_buffer
                };

                self.ppu.data_buffer = val;

                ret
            }
            _ => unreachable!(),
        }
    }

    pub fn ppu_write_reg(&mut self, reg: u8, val: u8) {
        assert!(reg < 8);
        match reg {
            PPUSTATUS => {}
            PPUCTRL => {
                self.ppu.control_reg = ControlReg::from_bytes([val]);
                self.ppu
                    .temp_vram_address
                    .set_nametable_x(self.ppu.control_reg.nametable_x());
                self.ppu
                    .temp_vram_address
                    .set_nametable_y(self.ppu.control_reg.nametable_y());
            }
            PPUMASK => {
                self.ppu.mask_reg = MaskReg::from_bytes([val]);
            }
            PPUADDR => {
                if self.ppu.second_byte {
                    self.ppu.temp_vram_address =
                        VRAMAddress::from_bytes([val, self.ppu.temp_vram_address.bytes[1]]);
                    self.ppu.vram_address = self.ppu.temp_vram_address.clone();
                    self.ppu.second_byte = false;
                } else {
                    self.ppu.temp_vram_address = VRAMAddress::from_bytes([0, val]);
                    self.ppu.second_byte = true;
                };
            }
            PPUDATA => {
                assert!(!self.ppu.second_byte);

                let mut temp_addr = u16::from_ne_bytes(self.ppu.vram_address.bytes);
                self.ppu_write(temp_addr, val);

                temp_addr += [1, 32][self.ppu.control_reg.vram_increment() as usize];
                self.ppu.vram_address = VRAMAddress::from_bytes(temp_addr.to_ne_bytes());
            }
            // TODO
            OAMADDR | OAMDATA => {}
            PPUSCROLL => {
                if self.ppu.second_byte {
                    self.ppu.temp_vram_address.set_coarse_y(val >> 3);
                    self.ppu.temp_vram_address.set_fine_y(val & 0b111);
                    self.ppu.second_byte = false;
                } else {
                    self.ppu.temp_vram_address.set_coarse_x(val >> 3);
                    self.ppu.fine_x = val & 0b111;
                    self.ppu.second_byte = true;
                }
            }
            _ => unreachable!(),
        }
    }

    fn ppu_read(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.mapper.read_ppu(addr).unwrap()
        } else if addr < 0x3000 {
            let rel = addr as usize - 0x2000;
            let nametable = rel / 0x400;
            let off = rel & 0x3FF;

            self.ppu.nametables[nametable][off]
        } else if addr < 0x3EFF {
            let rel = addr as usize - 0x3000;
            let nametable = rel / 0x400;
            let off = rel & 0x3FF;

            self.ppu.nametables[nametable][off]
        } else if addr < 0x3FFF {
            todo!()
        } else {
            unreachable!();
        }
    }

    fn ppu_write(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            self.mapper.write_ppu(addr, val).unwrap();
        } else if addr < 0x3000 {
            let rel = addr as usize - 0x2000;
            let nametable = rel / 0x400;
            let off = rel & 0x3FF;

            self.ppu.nametables[nametable][off] = val;
        } else if addr < 0x3EFF {
            let rel = addr as usize - 0x3000;
            let nametable = rel / 0x400;
            let off = rel & 0x3FF;

            self.ppu.nametables[nametable][off] = val;
        } else if addr < 0x3FFF {
            let off = addr & 0x1F;
            self.ppu.palette_table[off as usize] = val;
        } else {
            unreachable!();
        }
    }

    /// Retrieves a tile from the pattern table
    fn get_sprite_line(&self, right: bool, idx: u8, fine_y: u8) -> (u8, u8) {
        assert!(fine_y < 8);

        let idx = idx as u16;
        let fine_y = fine_y as u16;

        let pattern_table_addr = if right { 0x1000 } else { 0x0 };
        let base_addr = pattern_table_addr + idx * 16 + fine_y;

        let (lsb_byte, msb_byte) = (self.ppu_read(base_addr), self.ppu_read(base_addr + 8));

        (lsb_byte, msb_byte)
    }

    fn increment_vram_x(&mut self) {
        if self.ppu.mask_reg.show_background() == 0 && self.ppu.mask_reg.show_sprites() == 0 {
            return;
        }

        if self.ppu.vram_address.coarse_x() == 31 {
            self.ppu
                .vram_address
                .set_nametable_x((self.ppu.vram_address.nametable_x() == 1).not().into());
            self.ppu.vram_address.set_coarse_x(0);
        } else {
            self.ppu
                .vram_address
                .set_coarse_x(self.ppu.vram_address.coarse_x() + 1);
        }
    }

    fn increment_vram_y(&mut self) {
        if self.ppu.mask_reg.show_background() == 0 && self.ppu.mask_reg.show_sprites() == 0 {
            return;
        }

        if self.ppu.vram_address.fine_y() < 7 {
            self.ppu
                .vram_address
                .set_fine_y(self.ppu.vram_address.fine_y() + 1);
        } else {
            self.ppu.vram_address.set_fine_y(0);
            let mut coarse_y = self.ppu.vram_address.coarse_y();

            if coarse_y == 29 {
                coarse_y = 0;
                self.ppu
                    .vram_address
                    .set_nametable_y((self.ppu.vram_address.nametable_y() == 1).not().into());
            } else if coarse_y == 31 {
                coarse_y = 0;
            } else {
                coarse_y += 1;
            }

            self.ppu.vram_address.set_coarse_y(coarse_y);
        }
    }

    fn transfer_vram_x(&mut self) {
        if self.ppu.mask_reg.show_background() == 0 && self.ppu.mask_reg.show_sprites() == 0 {
            return;
        }

        self.ppu
            .vram_address
            .set_coarse_x(self.ppu.temp_vram_address.coarse_x());
        self.ppu
            .vram_address
            .set_nametable_x(self.ppu.temp_vram_address.nametable_x());
    }

    fn transfer_vram_y(&mut self) {
        if self.ppu.mask_reg.show_background() == 0 && self.ppu.mask_reg.show_sprites() == 0 {
            return;
        }

        self.ppu
            .vram_address
            .set_coarse_y(self.ppu.temp_vram_address.coarse_y());
        self.ppu
            .vram_address
            .set_nametable_y(self.ppu.temp_vram_address.nametable_y());
        self.ppu
            .vram_address
            .set_fine_y(self.ppu.temp_vram_address.fine_y());
    }

    pub fn clock_ppu(&mut self) {
        if self.ppu.scanline == 0 && self.ppu.cycle == 0 {
            self.ppu.vertical_blanking = false;
        }

        if self.ppu.scanline < 240 {
            match self.ppu.cycle.cmp(&256) {
                std::cmp::Ordering::Equal => {
                    self.increment_vram_y();
                }
                std::cmp::Ordering::Less => {
                    let x = self.ppu.cycle;
                    let y = self.ppu.scanline;

                    if x != 0 && x % 8 == 0 {
                        self.increment_vram_x();
                        let vram_addr = u16::from_ne_bytes(self.ppu.vram_address.bytes);

                        let sprite_addr = 0x2000 | vram_addr & 0xFFF;
                        self.ppu.current_sprite_index = self.ppu_read(sprite_addr);

                        let (lsb, msb) = self.get_sprite_line(
                            self.ppu.control_reg.background_pattern_table_address() > 0,
                            self.ppu.current_sprite_index,
                            self.ppu.vram_address.fine_y(),
                        );

                        self.ppu.lsb_shift_reg = lsb;
                        self.ppu.msb_shift_reg = msb;

                        let attrib_addr = 0x23C0
                            | 0b1111000000
                            | (self.ppu.vram_address.coarse_x() >> 2) as u16
                            | ((self.ppu.vram_address.coarse_y() >> 2) << 3) as u16
                            | (self.ppu.vram_address.nametable_x() as u16) << 10
                            | (self.ppu.vram_address.nametable_y() as u16) << 11;

                        let mut attrib = self.ppu_read(attrib_addr);
                        if self.ppu.vram_address.coarse_y() & 0b10 > 0 {
                            attrib >>= 4;
                        }

                        if self.ppu.vram_address.coarse_x() & 0b10 > 0 {
                            attrib >>= 2;
                        }

                        self.ppu.current_palette = attrib & 0b11;
                    }

                    let color_idx =
                        (self.ppu.msb_shift_reg >> 7) * 2 + (self.ppu.lsb_shift_reg >> 7);
                    self.ppu.msb_shift_reg <<= 1;
                    self.ppu.lsb_shift_reg <<= 1;

                    let palette_table_idx = self.ppu.current_palette * 4 + color_idx;

                    let idx = self.ppu.palette_table[palette_table_idx as usize] as usize;

                    let (r, g, b) = (PALETTE[idx * 3], PALETTE[idx * 3 + 1], PALETTE[idx * 3 + 2]);

                    self.draw_pixel(x, y, Color::RGB(r, g, b));
                }
                _ => {}
            }
        }

        if self.ppu.cycle == 257 {
            self.transfer_vram_x();
        }

        if self.ppu.scanline == 261 && self.ppu.cycle >= 280 && self.ppu.cycle <= 304 {
            self.transfer_vram_y();
        }

        self.ppu.cycle += 1;
        if self.ppu.cycle > 340 {
            self.ppu.cycle = 0;
            self.ppu.scanline += 1;
        }

        if self.ppu.scanline == 241 && self.ppu.cycle == 0 {
            self.ppu.vertical_blanking = true;
            if self.ppu.control_reg.generate_nmi() > 0 {
                self.nmi();
            }
            self.canvas.present();
        }

        if self.ppu.scanline > 261 {
            self.ppu.scanline = 0;
            self.frame_complete = true;
            println!("FRAME DONE");
        }
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.canvas.set_draw_color(color);
        if WINDOW_SCALE == 1 {
            let point = Point::new(x as i32, y as i32);
            self.canvas.draw_point(point).unwrap();
        } else {
            let rect = Rect::new(
                x as i32 * WINDOW_SCALE as i32,
                y as i32 * WINDOW_SCALE as i32,
                WINDOW_SCALE,
                WINDOW_SCALE,
            );
            self.canvas.fill_rect(rect).unwrap();
        }
    }
}
