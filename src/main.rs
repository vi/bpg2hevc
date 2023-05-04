
use std::{path::PathBuf, io::Write};

use binrw::{BinRead};
use bitstream_io::{BigEndian, BitWriter, BitWrite, Endianness};
use bpg2hevc::Bpg;

pub fn write_egc<W:Write,E:Endianness>(w: &mut BitWriter<W,E>, mut x: u32) -> std::io::Result<()> {
    x+=1;
    let l = x.ilog2();
    for _ in 0..l {
        w.write_bit(false)?;
    }
    for j in 0..=l {
        w.write_bit(x & (0x01 << (l-j)) != 0)?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        /// BPG file to read and convert to HEVC raw stream to stdout.
        /// `MP4Box -add-image w.hvc:primary -new w.heic` would help to pack it as a HEIF image.
        required path: PathBuf
    };

    let mut f = std::fs::File::open(flags.path)?;
    let bpg = Bpg::read(&mut f)?;
    //eprintln!("{:#?}", bpg);

    let mut so = std::io::stdout();
    so.write_all(b"\x00\x00\x01\x40\x01\x0c\x01\xff\xff\x03\x70\x00\x00\x03\x00\x90\x00\x00\x03\x00\x00\x03\x00\x1e\xaa\x02\x40")?;
               
    //so.write_all(b"\x00\x00\x01\x42\x01\x01\x03\x70\x00\x00\x03\x00\x90\x00\x00\x03\x00\x00\x03\x00\x1e\xa0\x34\x81\x85\x96\xaa\x49\x1b\x6b\x80\x40\x00\x00\x03\x00\x40\x00\x00\x06\x42")?;
    let mut b = BitWriter::<_, BigEndian>::new(so);

    let sps1 = b"000000000000000000000000000000010100001000000001000000010000001101110000000000000000000000000011000000001001000000000000000000000000001100000000000000000000001100000000011110001010";
    for x in sps1 {
        b.write_bit(*x == b'1')?;
    }
    let pic_width_in_luma_samples = (bpg.picture_width.0 + 7) / 8 * 8;
    let pic_height_in_luma_samples = (bpg.picture_height.0 + 7) / 8 * 8;
    write_egc(&mut b, pic_width_in_luma_samples)?;
    write_egc(&mut b, pic_height_in_luma_samples)?;
    // 0000000000110110100010000000001111000001

    let sps2 = b"01100101101010101001001001000110110110101110000000010000000000000000000000110000000000000001000000000000000000000011000000000001100100001000";
    for x in sps2 {
        b.write_bit(*x == b'1')?;
    }

    b.byte_align()?;
    let mut so = b.into_writer();
                  
    so.write_all(b"\x00\x00\x01")?;
    std::io::copy(&mut f, &mut so)?;
    Ok(())
}
