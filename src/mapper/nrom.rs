use super::Mapper;

enum NROMMapperType {
    NROM128,
    NROM256,
}

// https://www.nesdev.org/wiki/NROM
pub struct NROMMapper {
    typ: NROMMapperType,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl NROMMapper {
    fn translate_prg_address(&self, addr: u16) -> u16 {
        match self.typ {
            NROMMapperType::NROM128 => addr & 0x3FFF,
            NROMMapperType::NROM256 => addr & 0x7FFF,
        }
    }
}

impl Mapper for NROMMapper {
    fn new(file_buff: &[u8], nes_file: &crate::nes::NESFile) -> Self {
        let prg_rom_size = nes_file.prg_rom_size as usize * usize::pow(2, 14);
        // TODO: trainer
        let prg_rom_start = 16;
        let prg_rom_end = prg_rom_start + prg_rom_size;

        let chr_rom_size = nes_file.chr_rom_size as usize * usize::pow(2, 13);
        let chr_rom_start = prg_rom_end;
        let chr_rom_end = chr_rom_start + chr_rom_size;

        let prg_rom = Vec::from(&file_buff[prg_rom_start..prg_rom_end]);
        let chr_rom = Vec::from(&file_buff[chr_rom_start..chr_rom_end]);

        Self {
            typ: if nes_file.prg_rom_size == 1 {
                NROMMapperType::NROM128
            } else {
                NROMMapperType::NROM256
            },
            prg_rom,
            chr_rom,
        }
    }

    fn read_cpu(&self, addr: u16) -> Result<u8, ()> {
        if addr < 0x8000 {
            return Err(());
        }

        let addr = self.translate_prg_address(addr);
        Ok(self.prg_rom[addr as usize])
    }

    fn write_cpu(&mut self, addr: u16, val: u8) -> Result<(), ()> {
        if addr < 0x8000 {
            return Err(());
        }

        let addr = self.translate_prg_address(addr);
        self.prg_rom[addr as usize] = val;
        Ok(())
    }

    fn read_ppu(&self, addr: u16) -> Result<u8, ()> {
        if addr >= 0x2000 {
            return Err(());
        }

        Ok(self.chr_rom[addr as usize])
    }

    fn write_ppu(&mut self, addr: u16, val: u8) -> Result<(), ()> {
        if addr >= 0x2000 {
            return Err(());
        }

        self.chr_rom[addr as usize] = val;
        Ok(())
    }

    fn entrypoint(&self) -> u16 {
        0xC000
    }
}
