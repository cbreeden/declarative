use decl::Declarative;

macro_rules! declare_primitives {
    ($($func:path => $final:ident, $size:expr),* $(,)*) => {
        $(
            impl<'buf> Declarative<'buf> for $final {
                fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
                    if buffer.len() < Self::SIZE {
                        return Err(Error::InsufficientBytes);
                    }

                    let dest = $func(buffer);
                    Ok((dest, &buffer[Self::SIZE..]))
                }
            }

            impl StaticEncodingSize for $final {
                const SIZE: usize = $size;
            }
        )*
    };
}

declare_primitives!(
    read_u8      => u8,  1,
    read_i8      => i8,  1,
    BE::read_u16 => u16, 2,
    BE::read_i16 => i16, 2,
    BE::read_u32 => u32, 4,
    BE::read_i32 => i32, 4,
    BE::read_u64 => u64, 8,
    BE::read_i64 => i64, 8,
);