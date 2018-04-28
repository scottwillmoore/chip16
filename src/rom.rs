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

/*
use nom::{be_u16, be_u32, be_u8};

named!(
    parse_version<Version>,
    bits!(do_parse!(
        major: take_bits!(u8, 4) >> minor: take_bits!(u8, 4) >> (Version(major, minor))
    ))
);

named!(
    parse_raw<Rom>,
    do_parse!(
        (Rom {
            format: RomFormat::Raw,
            version: None,
            size: 0,
            start_address: 0,
            contents: vec![],
        })
    )
);

named!(
    parse_rom<Rom>,
    do_parse!(
        tag!("CH16") >> version: parse_version >> size: be_u32 >> start_address: be_u16
            >> checksum: be_u32 >> (Rom {
            format: RomFormat::Chip16,
            version: Some(version),
            size: size,
            start_address: start_address,
            contents: vec![],
        })
    )
);
*/

impl Rom {
    pub fn read<R: Read>(reader: &mut R) -> Rom {
        let mut data = vec![0; 16];
        reader.read_to_end(&mut data).unwrap();

        if &data[..4] == MAGIC_NUMBER {
            Rom::read_chip16(data)
        } else {
            Rom::read_raw(data)
        }
    }

    fn read_raw(data: Vec<u8>) -> Rom {
        Rom {
            format: RomFormat::Raw,
            version: None,
            size: data.len() as u32,
            start_address: 0,
            contents: data,
        }
    }

    // NOTE: Might be cleaner when using the nom crate.
    fn read_chip16(data: Vec<u8>) -> Rom {
        assert!(data.len() > 16);

        let version_major = data[5] & 0xF0 >> 4;
        let version_minor = data[5] & 0x0F;

        let size = (&data[6..10]).read_u32::<LittleEndian>().unwrap();
        let start_address = (&data[10..12]).read_u16::<LittleEndian>().unwrap();
        let checksum = (&data[12..16]).read_u32::<LittleEndian>().unwrap();

        let contents = data[16..(16 + size as usize)].to_vec();

        let mut digest = crc32::Digest::new(CRC32_POLYNOMIAL);
        digest.write(&contents[..]);
        // TODO: Return a Result instead of panic.
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
