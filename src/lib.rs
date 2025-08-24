mod reader;

use reader::DataReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct GBBinary {
    header: Header,
}

#[derive(Serialize)]
pub enum LicenseeCode {
    None,
    Unknown,
    Nintendo,
    Capcom,
    Bandai,
    Namco,
}

#[derive(Serialize)]
pub enum GBCFlag {
    /// Not explictely set, only support the GameBoy Classic
    GBOnly,
    /// support the GameBoy Color and the GameBoy Classic
    GBCAndGB,
    /// only supports the GameBoy Color
    GBCOnly,
}

#[derive(Serialize)]
pub enum SGBFlag {
    NoSGB,
    SGBSupport,
}

#[derive(Serialize)]
pub enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1xRAM,
    MBC1xRAMxBattery,
    MBC2,
    MBC2xBattery,
    ROMxRAM,
    ROMxRAMxBattery,
    MMM01,
    MMM01xRAM,
    MMM01xRAMxBattery,
    MBC3xTimerxBattery,
    MBC3xTimerxRAMxBattery,
    MBC3,
    MBC3xRAM,
    MBC3xRAMxBattery,
    MBC5,
    MBC5xRAM,
    MBC5xRAMxBattery,
    MBC5xRumble,
    MBC5xRumblexRAM,
    MBC5xRumblexRAMxBattery,
    MBC6,
    MBC7xSensorxRumblexRAMxBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1xRAMxBattery,
}

#[derive(Serialize)]
pub enum ROMSize {
    NoBanking,
    Banks4,
    Banks8,
    Banks16,
    Banks32,
    Banks64,
    Banks72,
    Banks80,
    Banks96,
    Banks128,
    Banks256,
    Banks512,
}

#[derive(Serialize)]
pub enum RAMSize {
    None,
    KB2,
    KB8,
    KB32,
    KB64,
    KB128,
}

#[derive(Serialize)]
pub enum DestinationCode {
    Japanese,
    NonJapanese,
}

#[derive(Serialize)]
pub struct Header {
    pub game_title: String,
    pub manufacturer_code: String,
    pub gbc_flag: GBCFlag,
    pub licensee_code: LicenseeCode,
    pub sgb_flag: SGBFlag,
    pub cartridge_type: CartridgeType,
    pub rom_size: ROMSize,
    pub ram_size: RAMSize,
    pub destination_code: DestinationCode,
    pub rom_version: u8,
    pub checksum: u8,
    pub global_checksum: u16,
}

const NEW_LICENCSEE_CODE_VAL: u8 = 0x33;

pub fn load(data: &[u8]) -> Result<GBBinary, String> {
    let mut reader = DataReader::new(data);
    parse_vectors(&mut reader)?;
    let header = parse_header(&mut reader)?;

    Ok(GBBinary { header })
}

fn parse_vectors(reader: &mut DataReader) -> Result<(), String> {
    reader.skip(0xFF);
    Ok(())
}

fn parse_header(reader: &mut DataReader) -> Result<Header, String> {
    reader.skip(4); // entry point
    reader.skip(49); // logo data

    let old_licensee_code = reader.read_u8_at(0x14B);

    let game_title = if old_licensee_code == NEW_LICENCSEE_CODE_VAL {
        clean_string(&reader.read_utf8_string(11))
    } else {
        clean_string(&reader.read_utf8_string(15))
    };

    let mut manufacturer_code = "".to_string();
    if old_licensee_code == NEW_LICENCSEE_CODE_VAL {
        manufacturer_code = clean_string(&reader.read_utf8_string(4));
    }

    let gbc_flag = parse_gbc_flag(reader.read_u8())?;
    let new_licensee_code = [reader.read_u8(), reader.read_u8()];
    let licensee_code = if old_licensee_code == NEW_LICENCSEE_CODE_VAL {
        parse_new_licensee_code(&new_licensee_code)
    } else {
        parse_old_licensee_code(old_licensee_code)
    };
    let sgb_flag = parse_sgb_flag(reader.read_u8())?;
    let cartridge_type = parse_cartridge_type(reader.read_u8())?;
    let rom_size = parse_rom_size(reader.read_u8())?;
    let ram_size = parse_ram_size(reader.read_u8())?;
    let destination_code = parse_destination_code(reader.read_u8())?;
    reader.skip(1); // old licensee code already read above
    let rom_version = reader.read_u8();
    let checksum = reader.read_u8();
    let global_checksum = reader.read_u16();

    Ok(Header {
        game_title,
        manufacturer_code,
        gbc_flag,
        licensee_code,
        sgb_flag,
        cartridge_type,
        rom_size,
        ram_size,
        destination_code,
        rom_version,
        checksum,
        global_checksum,
    })
}

fn parse_gbc_flag(flag: u8) -> Result<GBCFlag, String> {
    match flag {
        0 => Ok(GBCFlag::GBOnly),
        0x80 => Ok(GBCFlag::GBCAndGB),
        0xC0 => Ok(GBCFlag::GBCOnly),
        _ => Err(format!("unsupported GBC flag: {:x}", flag)),
    }
}

fn parse_sgb_flag(flag: u8) -> Result<SGBFlag, String> {
    match flag {
        0x00 => Ok(SGBFlag::NoSGB),
        0x03 => Ok(SGBFlag::SGBSupport),
        _ => Err(format!("unsupported SGB flag: {:x}", flag)),
    }
}

fn parse_cartridge_type(t: u8) -> Result<CartridgeType, String> {
    match t {
        0x00 => Ok(CartridgeType::ROMOnly),
        0x01 => Ok(CartridgeType::MBC1),
        0x02 => Ok(CartridgeType::MBC1xRAM),
        0x03 => Ok(CartridgeType::MBC1xRAMxBattery),
        0x05 => Ok(CartridgeType::MBC2),
        0x06 => Ok(CartridgeType::MBC2xBattery),
        0x08 => Ok(CartridgeType::ROMxRAM),
        0x09 => Ok(CartridgeType::ROMxRAMxBattery),
        0x0B => Ok(CartridgeType::MMM01),
        0x0C => Ok(CartridgeType::MMM01xRAM),
        0x0D => Ok(CartridgeType::MMM01xRAMxBattery),
        0x0F => Ok(CartridgeType::MBC3xTimerxBattery),
        0x10 => Ok(CartridgeType::MBC3xTimerxRAMxBattery),
        0x11 => Ok(CartridgeType::MBC3),
        0x12 => Ok(CartridgeType::MBC3xRAM),
        0x13 => Ok(CartridgeType::MBC1xRAMxBattery),
        0x19 => Ok(CartridgeType::MBC5),
        0x1A => Ok(CartridgeType::MBC5xRAM),
        0x1B => Ok(CartridgeType::MBC5xRAMxBattery),
        0x1C => Ok(CartridgeType::MBC5xRumble),
        0x1D => Ok(CartridgeType::MBC5xRumblexRAM),
        0x1E => Ok(CartridgeType::MBC5xRumblexRAMxBattery),
        0x20 => Ok(CartridgeType::MBC6),
        0x22 => Ok(CartridgeType::MBC7xSensorxRumblexRAMxBattery),
        0xFC => Ok(CartridgeType::PocketCamera),
        0xFD => Ok(CartridgeType::BandaiTama5),
        0xFE => Ok(CartridgeType::HuC3),
        0xFF => Ok(CartridgeType::HuC1xRAMxBattery),
        _ => Err(format!("unsuported cartridge type: {:x}", t)),
    }
}

fn parse_rom_size(v: u8) -> Result<ROMSize, String> {
    match v {
        0x00 => Ok(ROMSize::NoBanking),
        0x01 => Ok(ROMSize::Banks4),
        0x02 => Ok(ROMSize::Banks8),
        0x03 => Ok(ROMSize::Banks16),
        0x04 => Ok(ROMSize::Banks32),
        0x05 => Ok(ROMSize::Banks64),
        0x06 => Ok(ROMSize::Banks128),
        0x07 => Ok(ROMSize::Banks256),
        0x08 => Ok(ROMSize::Banks512),
        0x52 => Ok(ROMSize::Banks72),
        0x53 => Ok(ROMSize::Banks80),
        0x54 => Ok(ROMSize::Banks96),
        _ => Err(format!("unsupported rom size: {:x}", v)),
    }
}

fn parse_ram_size(v: u8) -> Result<RAMSize, String> {
    match v {
        0x00 => Ok(RAMSize::None),
        0x01 => Ok(RAMSize::KB2),
        0x02 => Ok(RAMSize::KB8),
        0x03 => Ok(RAMSize::KB32),
        0x04 => Ok(RAMSize::KB128),
        0x05 => Ok(RAMSize::KB64),
        _ => Err(format!("unsupported ram size: {:x}", v)),
    }
}

fn parse_destination_code(v: u8) -> Result<DestinationCode, String> {
    match v {
        0x00 => Ok(DestinationCode::Japanese),
        0x01 => Ok(DestinationCode::NonJapanese),
        _ => Err(format!("unsupported destination code: {:x}", v)),
    }
}

fn parse_new_licensee_code(code: &[u8; 2]) -> LicenseeCode {
    match code {
        b"00" => LicenseeCode::None,
        b"01" => LicenseeCode::Nintendo,
        b"08" => LicenseeCode::Capcom,
        b"B2" => LicenseeCode::Bandai,
        b"AF" => LicenseeCode::Namco,
        _ => LicenseeCode::Unknown,
    }
    // TODO complete this mapping
}

fn parse_old_licensee_code(code: u8) -> LicenseeCode {
    match code {
        0x0 => LicenseeCode::None,
        0x01 => LicenseeCode::Nintendo,
        0x08 => LicenseeCode::Capcom,
        0xB2 => LicenseeCode::Bandai,
        0xAF => LicenseeCode::Namco,
        _ => LicenseeCode::Unknown,
    }
    // TODO complete this mapping
}

fn clean_string(str: &str) -> String {
    str.replace('\0', "")
}
