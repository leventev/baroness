use super::Mapper;

enum NROMMapperType {
    NROM128,
    NROM256,
}

// https://www.nesdev.org/wiki/NROM
pub struct NROMMapper {
    typ: NROMMapperType,
    prg_rom: Vec<u8>,
}

impl NROMMapper {
    fn translate_address(&self, addr: u16) -> u16 {
        match self.typ {
            NROMMapperType::NROM128 => addr & 0x3FFF,
            NROMMapperType::NROM256 => addr & 0x7FFF,
        }
    }
}

impl Mapper for NROMMapper {
    fn new(file_buff: &[u8], nes_file: &crate::nes::NESFile) -> Self {
        let prg_rom_size = nes_file.prg_rom_size as usize * usize::pow(2, 14);
        let prg_rom_start = 16;
        let prg_rom_end = prg_rom_start + prg_rom_size;

        let program_rom = &file_buff[prg_rom_start..prg_rom_end];

        let prg_rom = Vec::from(program_rom);

        Self {
            typ: if nes_file.prg_rom_size == 1 {
                NROMMapperType::NROM128
            } else {
                NROMMapperType::NROM256
            },
            prg_rom,
        }
    }

    fn read(&self, addr: u16) -> Result<u8, ()> {
        if addr < 0x8000 {
            return Err(());
        }

        let addr = self.translate_address(addr);
        Ok(self.prg_rom[addr as usize])
    }

    fn write(&mut self, addr: u16, val: u8) -> Result<(), ()> {
        if addr < 0x8000 {
            return Err(());
        }

        let addr = self.translate_address(addr);
        self.prg_rom[addr as usize] = val;
        Ok(())
    }

    fn entrypoint(&self) -> u16 {
        0x8000
    }
}
