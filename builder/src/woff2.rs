//! I'm experimenting with a WOFF2 parser, since it'd allow me to trim fonts on the fly
//! Run the woff2 test to see progress
//!
//! Sources:
//! - A JS library called FontKit: https://github.com/foliojs/fontkit
//! - The official spec: https://www.w3.org/TR/WOFF2/

#![allow(dead_code)]

use core::mem;

const FONT: &[u8] = include_bytes!("../../fonts/material-icons.woff2");

#[test]
fn woff2() {
    let header: *const Woff2Header = unsafe { mem::transmute(FONT as *const [u8] as *const ()) };
    let header = unsafe { &*header };
    assert_eq!(header.signature, *b"wOF2");
    let num_tables = header.num_tables;
    let flavour = header.flavour;
    println!("Font has {} tables", num_tables);
    println!("Font's flavour is {flavour}");
    if flavour == 0x74746366 {
        panic!("Collection fonts not yet supported");
    }
    let mut table_bytes = &FONT[mem::size_of::<Woff2Header>()..];
    for _ in 0..num_tables {
        let (table, new_base) = TableDirectoryEntry::from_stream(table_bytes);
        table_bytes = &table_bytes[new_base..];
        println!("Table's flag is {:?}", table.flags);
    }
}

#[repr(packed)]
struct Woff2Header {
    /// 'wOF2'
    signature: [u8; 4],
    flavour: u32,
    length: u32,
    num_tables: u16,
    reserved: u16,
    total_size_uncompressed: u32,
    compressed_block_size: u32,
    major_version: u16,
    minor_version: u16,
    meta_offset: u32,
    meta_length: u32,
    meta_length_uncompressed: u32,
    priv_offset: u32,
    priv_length: u32,
}
#[repr(packed)]
struct TableDirectoryEntry {
    flags: TableFlag,
    tag: u32,
    original_length: UIntBase128,
    transform_length: UIntBase128,
}
impl TableDirectoryEntry {
    pub fn from_stream(bytes: &[u8]) -> (Self, usize) {
        let flags = bytes[0] >> 3;
        let tag = u32::from_be_bytes(bytes[1..5].try_into().unwrap());

        let bytes = &bytes[5..];
        let (original_length, new_base) = UIntBase128::from_stream(bytes);
        let bytes = &bytes[new_base..];
        let (transform_length, final_byte) = UIntBase128::from_stream(bytes);

        let this = Self {
            flags: unsafe { mem::transmute(flags) },
            tag,
            original_length,
            transform_length,
        };
        (this, 1 + 4 + new_base + final_byte)
    }
}

struct UIntBase128(u32);
impl UIntBase128 {
    pub fn from_stream(bytes: &[u8]) -> (Self, usize) {
        let mut val = 0;
        for (idx, byte) in bytes.iter().enumerate() {
            val = (val << 7) | (byte & 0b0111_1111) as u32;

            if byte & 0b1000_0000 != 0b1000_0000 {
                return (Self(val), idx);
            }
        }

        unreachable!("Invalid WOFF2")
    }
}

#[repr(packed)]
struct GlyfTableHeader {
    reserved: u16,
    option_flags: u16,
    num_glyphs: u16,
    index_format: u16,
    n_contour_stream_size: u32,
    n_points_stream_size: u32,
    flag_stream_size: u32,
    glyph_stream_size: u32,
    composite_stream_size: u32,
    bbox_stream_size: u32,
    instruction_stream_size: u32,
}
struct GlyfTable {}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
enum TableFlag {
    Cmap = 0,
    Head = 1,
    Hhea = 2,
    Hmtx = 3,
    Maxp = 4,
    Name = 5,
    Os2 = 6,
    Post = 7,
    Cvt = 8,
    Fpgm = 9,
    Glyf = 10,
    Loca = 11,
    Prep = 12,
    Cff = 13,
    Vorg = 14,
    Ebdt = 15,
    Eblc = 16,
    Gasp = 17,
    Hdmx = 18,
    Kern = 19,
    Ltsh = 20,
    Pclt = 21,
    Vdmx = 22,
    Vhea = 23,
    Vmtx = 24,
    Base = 25,
    Gdef = 26,
    Gpos = 27,
    Gsub = 28,
    Ebsc = 29,
    Jstf = 30,
    Math = 31,
    Cbdt = 32,
    Cblc = 33,
    Colr = 34,
    Cpal = 35,
    Svg = 36,
    Sbix = 37,
    Acnt = 38,
    Avar = 39,
    Bdat = 40,
    Bloc = 41,
    Bsln = 42,
    Cvar = 43,
    Fdsc = 44,
    LowerFeat = 45,
    Fmtx = 46,
    Fvar = 47,
    Gvar = 48,
    Hsty = 49,
    Just = 50,
    Lcar = 51,
    Mort = 52,
    Morx = 53,
    Opbd = 54,
    Prop = 55,
    Trak = 56,
    Zapf = 57,
    Silf = 58,
    Glat = 59,
    Gloc = 60,
    UpperFeat = 61,
    Sill = 62,
    Arbitrary = 63,
}
