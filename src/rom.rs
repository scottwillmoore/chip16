use byteorder::{LittleEndian, ReadBytesExt};
use crc::{Hasher32, crc32};
use failure::Error;
use std::io::Read;

const MAGIC_NUMBER: &[u8; 4] = b"CH16";
const CRC32_POLYNOMIAL: u32 = 0x04C11DB7;

pub enum RomFormat {
    Raw,
    Chip16,
}

pub struct Version(u8, u8);

pub struct Rom {
    pub format: RomFormat,
    pub version: Option<Version>,
    pub size: u32,
    pub start_address: u16,
    pub contents: Vec<u8>,
}

impl Rom {
    pub fn new<R: Read>(mut reader: R) -> Result<Rom, Error> {
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
        let version_major = version & 0xF0 >> 4;
        let version_minor = version & 0x0F;

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
