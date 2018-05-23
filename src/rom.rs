use byteorder::{LittleEndian, ReadBytesExt};
use crc::{Hasher32, crc32};
use failure::Error;
use std::io::Read;

const MAGIC_NUMBER: &[u8; 4] = b"CH16";
const CRC32_POLYNOMIAL: u32 = 0x04C11DB7;

#[derive(Debug, PartialEq)]
pub enum RomFormat {
    Raw,
    Chip16,
}

#[derive(Debug, PartialEq)]
pub struct Version(u8, u8);

#[derive(Debug)]
pub struct Rom {
    pub format: RomFormat,
    pub version: Option<Version>,
    pub size: u32,
    pub start_address: u16,
    pub contents: Vec<u8>,
}

impl Rom {
    pub fn new<R: Read>(mut reader: R) -> Result<Rom, Error> {
        // TODO: Use a BufReader.
        // Ensure this works for a raw format of length < 4.
        // Use the same method in the read_chip16 function for the header.
        let mut signature: [u8; 4] = [0; 4];
        reader.read_exact(&mut signature)?;

        match &signature {
            MAGIC_NUMBER => Rom::new_chip16(reader),
            _ => Rom::new_raw((&signature).chain(reader)),
        }
    }

    fn new_raw<R: Read>(mut reader: R) -> Result<Rom, Error> {
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents)?;

        Ok(Rom {
            format: RomFormat::Raw,
            version: None,
            size: contents.len() as u32,
            start_address: 0,
            contents,
        })
    }

    fn new_chip16<R: Read>(mut reader: R) -> Result<Rom, Error> {
        let reserved = reader.read_u8()?;
        ensure!(reserved == 0, "reserved from the header is non-zero");

        let version = reader.read_u8()?;
        let version_major = (version & 0xF0) >> 4;
        let version_minor = (version & 0x0F);

        let size = reader.read_u32::<LittleEndian>()?;
        let start_address = reader.read_u16::<LittleEndian>()?;
        ensure!(
            (start_address as u32) < size,
            "start address is not within the bounds of size from the header"
        );

        let checksum = reader.read_u32::<LittleEndian>()?;

        let mut contents = Vec::new();
        reader.take(size.into()).read_to_end(&mut contents)?;
        ensure!(
            (size as usize) == contents.len(),
            "the contents of the rom do not match the size from the header"
        );

        // NOTE: Until the crc crate gets updated we cannot compute the checksum.
        // let mut digest = crc32::Digest::new(CRC32_POLYNOMIAL);
        // digest.write(&contents[..]);
        // ensure!(
        //     digest.sum32() == checksum,
        //     "the computed checksum does not match the checksum from the header"
        // );

        Ok(Rom {
            format: RomFormat::Chip16,
            version: Some(Version(version_major, version_minor)),
            size,
            start_address,
            contents,
        })
    }
}

// TODO: Add test for wrong_checksum.
// TODO: Might be able to minimise line length by interpolating the header.
// E.g. [ ..header, 0x00, ...]. Does this work??? Or is there an equivalent.
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_rom {
        (
            $data:expr,
            $format:expr,
            $version:expr,
            $size:expr,
            $start_address:expr,
            $contents:expr
        ) => {
            let rom = Rom::new($data).unwrap();
            assert!(rom.format == $format);
            assert!(rom.version == $version);
            assert!(rom.size == $size);
            assert!(rom.start_address == $start_address);
            assert!(rom.contents == $contents);
        };
    }

    macro_rules! test_raw {
        ($func:ident, $data:expr) => {
            #[test]
            fn $func() {
                let size = $data.len() as u32;
                assert_rom!($data, RomFormat::Raw, None, size, 0, $data);
            }
        };
    }

    macro_rules! test_chip16 {
        ($func:ident, $version:expr, $size:expr, $start_address:expr, $data:expr) => {
            #[test]
            fn $func() {
                let contents = &$data[16..][..$size];
                assert_rom!(
                    $data,
                    RomFormat::Chip16,
                    Some($version),
                    $size,
                    $start_address,
                    contents
                );
            }
        };
    }

    macro_rules! test_chip16_error {
        ($func:ident, $data:expr) => {
            #[test]
            fn $func() {
                let result = Rom::new($data);
                assert!(result.is_err());
            }
        };
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_EMPTY: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,
    ];

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_ONE_BYTE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x00,
    ];

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_ONE_INSTRUCTION: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x00, 0x00, 0x00, 0x00,
    ];

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_MAZE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0xD8, 0x00,
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

    test_raw!(raw_empty, &ROM_EMPTY[16..]);
    test_raw!(raw_one_byte, &ROM_ONE_BYTE[16..]);
    test_raw!(raw_one_instruction, &ROM_ONE_INSTRUCTION[16..]);
    test_raw!(raw_maze, &ROM_MAZE[16..]);

    test_chip16!(chip16_empty, Version(1, 2), 0, 0, ROM_EMPTY);
    test_chip16!(chip16_one_byte, Version(1, 2), 1, 0, ROM_ONE_BYTE);
    test_chip16!(
        chip16_one_instruction,
        Version(1, 2),
        4,
        0,
        ROM_ONE_INSTRUCTION
    );
    test_chip16!(chip16_maze, Version(1, 2), 0xD8, 0, ROM_MAZE);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_INCOMPLETE_HEADER: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x00, 0x00,
    ];

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_NON_ZERO_RESERVED_BYTE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x01, 0x12, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,
    ];

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_START_ADDRESS_LARGER_THAN_SIZE: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x10, 0x00,
        0x00, 0x00, 0x10, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ROM_SIZE_LARGER_THAN_DATA: &[u8] = &[
        0x43, 0x48, 0x31, 0x36, 0x00, 0x12, 0x20, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xA7, 0x03, 0x1A, 0xC5,

        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    test_chip16_error!(chip16_incomplete_header, ROM_INCOMPLETE_HEADER);
    test_chip16_error!(chip16_non_zero_reserved_byte, ROM_NON_ZERO_RESERVED_BYTE);
    test_chip16_error!(
        chip16_start_address_larger_than_size,
        ROM_START_ADDRESS_LARGER_THAN_SIZE
    );
    test_chip16_error!(chip16_size_larger_than_data, ROM_SIZE_LARGER_THAN_DATA);
}
