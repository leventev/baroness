use crate::nes::NESFile;

use self::nrom::NROMMapper;

mod nrom;

pub trait Mapper {
    /// Instantiates a new mapper
    fn new(file_buff: &[u8], nes_file: &NESFile) -> Self
    where
        Self: Sized;

    /// Read from PRG memory
    fn read_cpu(&self, addr: u16) -> Result<u8, ()>;

    /// Write to PRG memory
    fn write_cpu(&mut self, addr: u16, val: u8) -> Result<(), ()>;

    /// Read from CHR memory
    fn read_ppu(&self, addr: u16) -> Result<u8, ()>;

    /// Write to CHR memory
    fn write_ppu(&mut self, addr: u16, val: u8) -> Result<(), ()>;

    /// Returns the entry point(beginning of PRG memory)
    /// FIXME: get it from RESET interrupt vector
    fn entrypoint(&self) -> u16;
}

pub fn get_mapper(file_buff: &[u8], nes_file: &NESFile) -> Box<dyn Mapper> {
    let mapper = match nes_file.mapper_number {
        0 => NROMMapper::new(file_buff, nes_file),
        _ => unreachable!(),
    };

    Box::new(mapper)
}
