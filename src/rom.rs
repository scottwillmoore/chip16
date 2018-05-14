use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use crc::{Hasher32, crc32};

const MAGIC_NUMBER: &[u8; 4] = b"CH16";
const CRC32_POLYNOMIAL: u32 = 0x04C11DB7;

enum RomFormat {
    Raw,
    Chip16,
}

struct Version(u8, u8);

pub struct Rom {
    format: RomFormat,
    version: Option<Version>,
    size: u32,
    start_address: u16,
    contents: Vec<u8>,
}

impl Rom {
    // TODO: Return a Result! Allow errors to be handled instead of panic.
    // NOTE: Will have to research best practice on combining various error types.
    // TODO: Consume the reader, instead of using a mutable reference.
    // NOTE: The read functions already completely consume the reader with read_to_end.
    // TODO: Is read the best function name for this constructor? Consider new, from, etc.
    pub fn read<R: Read>(reader: &mut R) -> Rom {
        let mut signature: [u8; 4] = [0; 4];
        reader.read_exact(&mut signature).unwrap();

        match &signature {
            MAGIC_NUMBER => Rom::read_chip16(reader),
            _ => Rom::read_raw(reader),
        }
    }

    fn read_raw<R: Read>(reader: &mut R) -> Rom {
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents).unwrap();

        let size = contents.len() as u32;

        Rom {
            format: RomFormat::Raw,
            version: None,
            size,
            start_address: 0,
            contents,
        }
    }

    fn read_chip16<R: Read>(reader: &mut R) -> Rom {
        let version = reader.read_u8().unwrap();
        let version_major = version & 0xF0 >> 4;
        let version_minor = version & 0x0F;

        let size = reader.read_u32::<LittleEndian>().unwrap();
        let start_address = reader.read_u16::<LittleEndian>().unwrap();
        let checksum = reader.read_u32::<LittleEndian>().unwrap();

        let mut contents = Vec::new();
        reader.read_to_end(&mut contents).unwrap();

        let mut digest = crc32::Digest::new(CRC32_POLYNOMIAL);
        digest.write(&contents[..]);
        assert!(digest.sum32() == checksum);

        Rom {
            format: RomFormat::Chip16,
            version: Some(Version(version_major, version_minor)),
            size,
            start_address,
            contents,
        }
    }
}
