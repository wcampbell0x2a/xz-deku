use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;

#[derive(DekuRead)]
pub struct Xz {
    magic: [u8; 6],
    stream_header: StreamHeader,
    block: Block,
}

#[derive(DekuRead)]
pub struct StreamHeader {
    stream_flags: Crc,
}

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum Crc {
    #[deku(id = "0x01")]
    Crc32([u8; 4]),

    #[deku(id = "0x04")]
    Crc64([u8; 8]),

    #[deku(id = "0x0a")]
    Sha256([u8; 32]),
}

#[derive(DekuRead)]
pub struct Block {
    header: BlockHeader,
}

#[derive(DekuRead)]
pub struct BlockHeader {
    size: u8,
    block_flags: BlockFlags,
    #[deku(skip, cond = "block_flags.compressed_size_field != 1")]
    compressed_size: Option<MultiByteInteger>,
    #[deku(skip, cond = "block_flags.uncompressed_size_field != 1")]
    uncompressed_size: Option<MultiByteInteger>,
    #[deku(count = "block_flags.num_of_filters")]
    filter_flags: Vec<FilterFlags>,
}

#[derive(DekuRead)]
pub struct BlockFlags {
    #[deku(bits = "2")]
    num_of_filters: u8,

    #[deku(bits = "4")]
    reserved: u8,

    #[deku(bits = "1")]
    compressed_size_field: u8,

    #[deku(bits = "1")]
    uncompressed_size_field: u8,
}

#[derive(DekuRead)]
pub struct FilterFlags {
    filter_id: MultiByteInteger,
    size_of_properties: MultiByteInteger,
    #[deku(count = "size_of_properties.0")]
    filter_properties: Vec<u8>,
}

#[derive(DekuRead)]
pub struct MultiByteInteger(#[deku(reader = "Self::read(deku::rest)")] u64);

impl MultiByteInteger {
    /// Read and convert to String
    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, u64), DekuError> {
        let mut i = 0;
        let mut num = 0;
        let mut mut_rest = rest;
        loop {
            let (rest, value) = u8::read(mut_rest, ())?;
            mut_rest = rest;
            i += 1;
            num |= ((value & 0x7f) as u64) << (i * 7);
            if value & 0x80 == 0 {
                break;
            }
        }
        Ok((mut_rest, num))
    }
}
