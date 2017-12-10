use std::fmt;

use byteorder::BE;
use byteorder::ByteOrder;

use declarative::DeclResult;
use declarative::StaticEncodingSize;
use declarative::Declarative;
use errors::Error;

fn read_u8(buf: &[u8]) -> u8 {
    buf[0]
}

fn read_i8(buf: &[u8]) -> i8 {
    buf[0] as i8
}

macro_rules! declare_types {
    ($($intermediate:tt => $final:ident),* $(,)*) => (
        $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $final(pub $intermediate);

        impl From<$intermediate> for $final {
            fn from(mid: $intermediate) -> $final {
                $final(mid)
            }
        }

        impl StaticEncodingSize for $final {
            const SIZE: usize = $intermediate::SIZE;
        }

        impl<'buf> Declarative<'buf> for $final {
            fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
                if buffer.len() < Self::SIZE {
                    return Err(Error::InsufficientBytes);
                }

                let (dest, buffer) = $intermediate::parse(buffer)?;
                Ok(($final(dest), buffer))
            }
        }
        )*
    );
}

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

declare_types!(
    i32 => Fixed,
    i16 => FWord,
    u16 => UFWord,
    i16 => F2Dot14,
    u64 => LongDateTime,
    u16 => Offset16,
    u32 => Offset32,
);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tag(pub [u8; 4]);

impl StaticEncodingSize for Tag {
    const SIZE: usize = 4;
}

impl Tag {
    fn as_u32(&self) -> u32 {
        (self.0[0] as u32) << 24 | (self.0[1] as u32) << 16 | (self.0[2] as u32) << 8
            | (self.0[3] as u32)
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        // Print the ASCII name if the contents are valid ASCII.
        // Otherwise, print hexidecimal digits.
        if self.0.iter().all(|&c| c >= 32 && c <= 126) {
            let s = str::from_utf8(&self.0[..]).unwrap();
            f.debug_tuple("Tag").field(&s).finish()
        } else {
            write!(f, "Tag(0x{:08X})", self.as_u32())
        }
    }
}

impl<'buf> Declarative<'buf> for Tag {
    fn parse(mut buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
        use std::io::Read;
        if buffer.len() < Self::SIZE {
            return Err(Error::InsufficientBytes);
        }

        let mut buf = [0u8; Self::SIZE];
        let _ = buffer.read_exact(&mut buf);
        return Ok((Tag(buf), buffer));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U24(pub u32);

impl StaticEncodingSize for U24 {
    const SIZE: usize = 3;
}

impl<'buf> Declarative<'buf> for U24 {
    fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
        if buffer.len() < Self::SIZE {
            return Err(Error::InsufficientBytes);
        }

        let dest = BE::read_u32(buffer);
        return Ok((U24(dest), &buffer[Self::SIZE..]));
    }
}
