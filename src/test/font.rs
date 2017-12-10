use errors::Error;
use array::Array;
use test::types::Offset32;
use test::types::Tag;
use declarative::DeclarativeArgsRead;
use declarative::DeclarativeRead;
use declarative::Declarative;
use declarative::StaticEncodingSize;
use declarative::DeclResult;

#[derive(Debug)]
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

#[derive(Debug)]
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
    // let mut data = &data[..];
    let font = (&data[..])
        .parse::<OffsetTable>()
        .expect("Failed to parse OffsetTable");

    println!("Font type: {:?}", font.version);
    println!("Number tables: {:?}", font.num_tables);
    println!("Search range: {:?}", font.search_range);
    println!("Entry selector: {:?}", font.entry_selector);
    println!("Range shift: {:?}", font.range_shift);

    for table in font.tables {
        let table = table.expect("failed to parse table");
        println!(
            "Tag: {:?}, check sum: 0x{:08x}, offset: 0x{:08x}, length: {:?}",
            table.tag,
            table.check_sum,
            table.offset.0,
            table.length
        );
    }
}
