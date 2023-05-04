#![allow(unused)]

use binrw::{binread,BinRead};
use modular_bitfield::{bitfield,specifiers::{B3,B4}};

#[binread]
#[br(big, magic = b"BPG\xFB")]
#[br(assert(!header1.alpha1_flag()))]
#[br(assert(!header1.alpha2_flag()))]
#[br(assert(!header1.limited_range_flag()))]
#[br(assert(!header1.animation_flag()))]
#[br(assert(header1.pixel_format()==0))]
#[br(assert(!header1.extension_present_flag()))]
#[br(assert(hevc_header_lenght.0==3))]
#[br(assert(hevc_header[0]==146))]
#[br(assert(hevc_header[1]==71))]
#[br(assert(hevc_header[2]==64))]
#[derive(Debug)]
pub struct Bpg {
    header1: BpgHeader1,
    pub picture_width : Ue7,
    pub picture_height  : Ue7,
    picture_data_length : Ue7,

    // no extensions, no alpha
    
    hevc_header_lenght: Ue7,
    #[br(count = hevc_header_lenght.0)]
    hevc_header: Vec<u8>,
}

#[bitfield]
#[derive(BinRead)]
#[br(map = Self::from_bytes)]
#[derive(Debug)]
pub struct BpgHeader1 {
    pixel_format : B3,
    alpha1_flag : bool,
    bit_depth_minus_8 : B4,

    color_space     : B4,
    extension_present_flag : bool,
    alpha2_flag       : bool,
    limited_range_flag    : bool,
    animation_flag    : bool,
}

/// Varint
#[derive(Debug)]
pub struct Ue7(pub u32);

impl binrw::BinRead for Ue7 {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut x = 0;
        loop {
            let mut b = u8::read_options(reader, endian, args)?;
            if b & 0x80 != 0 {
                b &= 0x7F;
                x <<= 7;
                x |= b as u32;
                continue;
            } else {
                x <<= 7;
                x |= b as u32;
                return  Ok(Ue7(x));
            }
        }
    }
}

#[test]
fn test_ue7() {
    use std::io::Cursor;
    assert_eq!(Ue7::read_be(&mut Cursor::new(b"\x08")).unwrap().0, 8);
    assert_eq!(Ue7::read_be(&mut Cursor::new(b"\x84\x1E")).unwrap().0, 542);
    assert_eq!(Ue7::read_be(&mut Cursor::new(b"\xAC\xBE\x17")).unwrap().0, 728855);
}
