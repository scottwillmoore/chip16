use byteorder::{LittleEndian, ReadBytesExt};
use crc::{Hasher32, crc32};
use failure::Error;
use std::io::Read;

const CRC32_POLYNOMIAL: u32 = 0x04C11DB7;

#[derive(Debug, PartialEq)]
pub enum RomFormat {
    Raw,
    Chip16,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Version(u8, u8);

impl From<u8> for Version {
    fn from(byte: u8) -> Version {
        let major = (byte & 0xF0) >> 4;
        let minor = byte & 0x0F;
        Version(major, minor)
    }
}

#[derive(Debug, PartialEq)]
pub struct Rom {
    pub format: RomFormat,
    pub version: Option<Version>,
    pub size: u32,
    pub start_address: u16,
    pub content: Vec<u8>,
}

impl Rom {
    pub fn new<R: Read>(mut reader: R) -> Result<Rom, Error> {
        let mut header = [0; 16];
        let bytes_read = reader.read(&mut header)?;
        ensure!(bytes_read >= 4, "the rom must be at least 4 bytes");

        let (signature, metadata) = (&header[..4], &header[4..]);
        match signature {
            b"CH16" => {
                ensure!(bytes_read == 16, "the header is not 16 bytes");
                Rom::decode_chip16(metadata, reader)
            }
            _ => {
                let content = header[..bytes_read].chain(reader);
                Rom::decode_raw(content)
            }
        }
    }

    fn decode_raw<R: Read>(mut reader: R) -> Result<Rom, Error> {
        let mut content = Vec::new();
        reader.read_to_end(&mut content)?;
        Ok(Rom {
            format: RomFormat::Raw,
            version: None,
            size: content.len() as u32,
            start_address: 0,
            content,
        })
    }

    fn decode_chip16<R: Read>(mut metadata: &[u8], mut reader: R) -> Result<Rom, Error> {
        let reserved = metadata.read_u8()?;
        ensure!(reserved == 0, "reserved is non-zero");

        let version = metadata.read_u8()?;
        let version = Some(version.into());

        let size = metadata.read_u32::<LittleEndian>()?;
        let start_address = metadata.read_u16::<LittleEndian>()?;
        ensure!(size >= 4, "the rom content must be at least 4 bytes");
        ensure!(
            size > start_address as u32,
            "start address is larger than size"
        );

        let checksum = metadata.read_u32::<LittleEndian>()?;

        let mut content = Vec::new();
        reader.take(size as u64).read_to_end(&mut content)?;
        ensure!(
            content.len() == size as usize,
            "the length of content is smaller than size"
        );

        // NOTE: Until the crc crate gets updated we cannot compute the checksum.
        // let mut digest = crc32::Digest::new(CRC32_POLYNOMIAL);
        // digest.write(&contents[..]);
        // ensure!(digest.sum32() == checksum, "the checksum is invalid");

        Ok(Rom {
            format: RomFormat::Chip16,
            version,
            size,
            start_address,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_assert_rom {
        ($func:ident, $reader:expr, $rom:expr) => {
            #[test]
            fn $func() {
                let rom = Rom::new($reader).unwrap();
                println!("{:?}", rom);
                assert!(rom == $rom);
            }
        };
    }

    macro_rules! test_assert_error {
        ($func:ident, $data:expr) => {
            #[test]
            fn $func() {
                let result = Rom::new($data);
                assert!(result.is_err());
            }
        };
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_ONE_INSTRUCTION: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x01, 0x02, 0x03, 0x04,
    ];

    test_assert_rom!(
        raw_one_instruction,
        &ROM_ONE_INSTRUCTION[16..],
        Rom {
            format: RomFormat::Raw,
            version: None,
            size: 4,
            start_address: 0,
            content: ROM_ONE_INSTRUCTION[16..(16 + 4)].to_vec(),
        }
    );

    test_assert_rom!(
        chip16_one_instruction,
        &ROM_ONE_INSTRUCTION[..],
        Rom {
            format: RomFormat::Chip16,
            version: Some(Version(1, 2)),
            size: 4,
            start_address: 0,
            content: ROM_ONE_INSTRUCTION[16..(16 + 4)].to_vec(),
        }
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_MAZE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x11, 0xD8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x02,
        0x03, 0x00, 0x01, 0x00, 0x20, 0x0A, 0x20, 0x00,
        0x20, 0x0B, 0x20, 0x00, 0x20, 0x00, 0x01, 0x00,
        0x20, 0x01, 0x01, 0x00, 0x20, 0x02, 0x00, 0x00,
        0x20, 0x03, 0x00, 0x00, 0x20, 0x04, 0x3E, 0x01,
        0x20, 0x05, 0x00, 0x00, 0x20, 0x06, 0xEE, 0x00,
        0x20, 0x0D, 0x01, 0x00, 0x20, 0x0E, 0x02, 0x00,
        0x20, 0x0F, 0x00, 0x00, 0x52, 0xA4, 0x02, 0x00,
        0x12, 0x00, 0x58, 0x00, 0x12, 0x09, 0x58, 0x00,
        0x52, 0x3A, 0x02, 0x00, 0x12, 0x00, 0x64, 0x00,
        0x12, 0x09, 0x64, 0x00, 0x10, 0x00, 0x6C, 0x00,
        0x24, 0x4A, 0x00, 0x00, 0x24, 0xF0, 0x00, 0x00,
        0x10, 0x00, 0x6C, 0x00, 0x24, 0x3A, 0x00, 0x00,
        0x24, 0xD0, 0x00, 0x00, 0x52, 0xB6, 0x02, 0x00,
        0x12, 0x00, 0x88, 0x00, 0x12, 0x09, 0x88, 0x00,
        0x52, 0x5B, 0x02, 0x00, 0x12, 0x00, 0x94, 0x00,
        0x12, 0x09, 0x94, 0x00, 0x10, 0x00, 0x9C, 0x00,
        0x24, 0x6B, 0x00, 0x00, 0x24, 0xF1, 0x00, 0x00,
        0x10, 0x00, 0x9C, 0x00, 0x24, 0x5B, 0x00, 0x00,
        0x24, 0xD1, 0x00, 0x00, 0x13, 0xF0, 0xA4, 0x00,
        0x10, 0x00, 0xAC, 0x00, 0x51, 0xEA, 0x00, 0x00,
        0x10, 0x00, 0xB0, 0x00, 0x41, 0xEA, 0x00, 0x00,
        0x13, 0xF1, 0xB8, 0x00, 0x10, 0x00, 0xC0, 0x00,
        0x51, 0xEB, 0x00, 0x00, 0x10, 0x00, 0xC4, 0x00,
        0x41, 0xEB, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x05, 0xBA, 0xD4, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x3C, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    test_assert_rom!(
        raw_maze,
        &ROM_MAZE[16..],
        Rom {
            format: RomFormat::Raw,
            version: None,
            size: 216,
            start_address: 0,
            content: ROM_MAZE[16..(16 + 216)].to_vec(),
        }
    );

    test_assert_rom!(
        chip16_maze,
        &ROM_MAZE[..],
        Rom {
            format: RomFormat::Chip16,
            version: Some(Version(1, 1)),
            size: 216,
            start_address: 0,
            content: ROM_MAZE[16..(16 + 216)].to_vec(),
        }
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_EMPTY: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,
    ];

    test_assert_error!(raw_empty, &ROM_EMPTY[16..]);
    test_assert_error!(chip16_empty, &ROM_EMPTY[..]);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_ONE_BYTE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x01,
    ];

    test_assert_error!(raw_one_byte, &ROM_ONE_BYTE[16..]);
    test_assert_error!(chip16_one_byte, &ROM_ONE_BYTE[..]);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_INCOMPLETE_HEADER: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x00, 0x00,
    ];

    test_assert_error!(chip16_incomplete_header, &ROM_INCOMPLETE_HEADER[..]);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_NON_ZERO_RESERVED_BYTE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x01, 0x12, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,
    ];

    test_assert_error!(
        chip16_non_zero_reserved_byte,
        &ROM_NON_ZERO_RESERVED_BYTE[..]
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_START_ADDRESS_LARGER_THAN_SIZE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x10, 0x00,
        0x00, 0x00, 0x10, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    test_assert_error!(
        chip16_start_address_larger_than_size,
        &ROM_START_ADDRESS_LARGER_THAN_SIZE[..]
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_SIZE_LARGER_THAN_DATA: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x20, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    test_assert_error!(chip16_size_larger_than_data, &ROM_SIZE_LARGER_THAN_DATA[..]);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_SIZE_IS_ZERO: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    test_assert_error!(chip16_size_is_zero, &ROM_SIZE_IS_ZERO[..]);
}
