use crate::nes::NESFile;

use self::nrom::NROMMapper;

mod nrom;

pub trait Mapper {
    fn new(file_buff: &[u8], nes_file: &NESFile) -> Self
    where
        Self: Sized;
    fn read(&self, addr: u16) -> Result<u8, ()>;
    fn write(&mut self, addr: u16, val: u8) -> Result<(), ()>;
    fn entrypoint(&self) -> u16;
}

pub fn get_mapper(file_buff: &[u8], nes_file: &NESFile) -> Box<dyn Mapper> {
    let mapper = match nes_file.mapper_number {
        0 => NROMMapper::new(file_buff, nes_file),
        _ => unreachable!(),
    };

    Box::new(mapper)
}
