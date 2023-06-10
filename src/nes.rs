use modular_bitfield::{bitfield, specifiers::{B1, B4}};

const NES_MAGIC: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

#[bitfield]
struct Flags6 {
    vertical_mirroring: B1,
    prg_ram: B1,
    trainer: B1,
    ignore_mirroring_control: B1,
    ignore: B4
}

pub enum MirroringMode {
    Horizontal,
    Vertical,
}

pub struct NESFile {
    /// in 16 KiB units
    pub prg_rom_size: u8,
    /// in 16 KiB units
    pub chr_rom_size: u8,

    ///
    pub mirroring_mode: MirroringMode,

    /// Cartridge contains battery-backed PRG RAM (0x6000-0x7FFF) or other persistent memory
    pub has_prg_ram: bool,

    /// 512-byte trainer at 0x7000-0x71FF (stored before PRG data)
    pub has_trainer: bool,

    /// Mapper number
    pub mapper_number: u8,
}

pub fn parse_nes_file(file: &[u8]) -> Result<NESFile, ()> {
    let magic = &file[..4];
    if magic != NES_MAGIC {
        return Err(());
    }

    let prg_rom_size = file[4];
    let chr_rom_size = file[5];
    let flags_6 = Flags6::from_bytes([file[6]]);
    let flags_7 = file[7];
    let _flags_8 = file[8];
    let _flags_9 = file[9];
    let _flags_10 = file[10];

    let nes = NESFile {
        prg_rom_size,
        chr_rom_size,
        mirroring_mode: if flags_6.vertical_mirroring() > 0 {
            MirroringMode::Vertical
        } else {
            MirroringMode::Horizontal
        },
        has_prg_ram: flags_6.prg_ram() > 0,
        has_trainer: flags_6.trainer() >0,
        mapper_number: flags_6.into_bytes()[0] >> 4 | flags_7 & 0b11110000,
    };

    Ok(nes)
}
