use errors::Error;
use array::Array;
use test::types::Offset32;
use test::types::Tag;
use declarative::DeclarativeArgsRead;
use declarative::DeclarativeRead;
use declarative::Declarative;
use declarative::StaticEncodingSize;
use declarative::DeclResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    OpenType,
    TrueType,
}

impl StaticEncodingSize for Version {
    const SIZE: usize = 4;
}

impl<'buf> Declarative<'buf> for Version {
    fn parse(buffer: &'buf [u8]) -> Result<(Self, &'buf [u8]), Error> {
        let (version, buffer) = u32::parse(buffer)?;
        let version = match version {
            0x00010000 => Version::TrueType,
            0x4F54544F => Version::OpenType,
            _ => return Err(Error::InvalidVersion),
        };

        Ok((version, buffer))
    }
}

#[derive(Debug)]
pub struct OffsetTable<'buf> {
    buffer: &'buf [u8],
    version: Version,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    tables: Array<'buf, TableRecord>,
}

impl<'buf> Declarative<'buf> for OffsetTable<'buf> {
    fn parse(mut buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
        let version = buffer.parse::<Version>()?;
        let num_tables = buffer.parse::<u16>()?;
        let search_range = buffer.parse::<u16>()?;
        let entry_selector = buffer.parse::<u16>()?;
        let range_shift = buffer.parse::<u16>()?;
        let tables = buffer.parse_array::<TableRecord>(num_tables as usize)?;

        Ok((
            OffsetTable {
                buffer,
                version,
                num_tables,
                search_range,
                entry_selector,
                range_shift,
                tables,
            },
            buffer,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct TableRecord {
    tag: Tag,
    check_sum: u32,
    offset: Offset32,
    length: u32,
}

impl StaticEncodingSize for TableRecord {
    const SIZE: usize = 16;
}

impl<'buf> Declarative<'buf> for TableRecord {
    fn parse(mut buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
        let tag = buffer.parse::<Tag>()?;
        let check_sum = buffer.parse::<u32>()?;
        let offset = buffer.parse::<Offset32>()?;
        let length = buffer.parse::<u32>()?;

        Ok((
            TableRecord {
                tag,
                check_sum,
                offset,
                length,
            },
            buffer,
        ))
    }
}

#[test]
fn test_stuff() {
    let data = open_font!("data/Roboto-Regular.ttf");
    let mut data = &data[..];
    let font = data
        .parse::<OffsetTable>()
        .expect("Failed to parse OffsetTable");

    macro_rules! assert_tables {
        ($font:expr, $($tag:expr, $checksum:expr, $offset:expr, $length:expr,)*) => (
            let mut tables = $font.tables.into_iter();
            $(
                let table = tables
                    .next()
                    .expect("insufficent number of tables")
                    .expect("failed to parse font table");

                assert_eq!(table.tag, $tag, "unexpected table tag");
                assert_eq!(table.check_sum, $checksum, "incorrect checksum");
                assert_eq!(table.offset, Offset32($offset), "incorrect offset");
                assert_eq!(table.length, $length, "incorrect table length");
            )*

            assert_eq!(tables.next(), None, "found more tables than expected");
        )
    }

    assert_eq!(font.version, Version::TrueType);
    assert_eq!(font.num_tables, 18);
    assert_eq!(font.search_range, 256);
    assert_eq!(font.entry_selector, 4);
    assert_eq!(font.range_shift, 32);

    assert_tables! (font,
        // Tag      Checksum     Offset      Length
        Tag(*b"GDEF"), 0xb442b082, 0x000228dc, 610,
        Tag(*b"GPOS"), 0xff1a12d7, 0x00022b40, 24012,
        Tag(*b"GSUB"), 0xeb82e459, 0x0002890c, 5520,
        Tag(*b"OS/2"), 0x9782b1a8, 0x000001a8, 96,
        Tag(*b"cmap"), 0x0177581e, 0x00001b58, 4678,
        Tag(*b"cvt "), 0x2ba8079d, 0x000030a8, 84,
        Tag(*b"fpgm"), 0x77f860ab, 0x00002da0, 444,
        Tag(*b"gasp"), 0x00080013, 0x000228d0, 12,
        Tag(*b"glyf"), 0x26ba0bf4, 0x00003b1c, 125292,
        Tag(*b"hdmx"), 0x557a607a, 0x00001640, 1304,
        Tag(*b"head"), 0xfc6ad27a, 0x0000012c, 54,
        Tag(*b"hhea"), 0x0aba0aae, 0x00000164, 36,
        Tag(*b"hmtx"), 0xae728f97, 0x00000208, 5176,
        Tag(*b"loca"), 0x8077ffbb, 0x000030fc, 2590,
        Tag(*b"maxp"), 0x073e0309, 0x00000188, 32,
        Tag(*b"name"), 0xe6a41589, 0x00022488, 1062,
        Tag(*b"post"), 0xff6d0064, 0x000228b0, 32,
        Tag(*b"prep"), 0xa266fac9, 0x00002f5c, 329,
    );
}