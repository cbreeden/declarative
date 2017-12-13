use errors::Error;
use array::Array;

use std::io::Read;
use std::fmt::Debug;

pub type DeclResult<'buf, T> = Result<(T, &'buf [u8]), Error>;

/// Implemented on types whose encoding size is known _prior_
/// to being parsed.
pub trait StaticEncodingSize {
    const SIZE: usize;
}

/// Implemented on types whose encoding size is known after
/// a the type has been decoded.
pub trait DynamicEncodingSize {
    fn size(&self) -> usize;
}

impl<T> DynamicEncodingSize for T
where
    T: StaticEncodingSize,
{
    fn size(&self) -> usize {
        Self::SIZE
    }
}

pub trait Declarative<'buf>: Sized {
    fn parse(&'buf [u8]) -> DeclResult<'buf, Self>;
}

pub trait DeclarativeWithArgs<'buf>: Sized {
    type Argument;
    fn parse_with(&'buf [u8], Self::Argument) -> DeclResult<'buf, Self>;
}

impl<'buf, T> DeclarativeWithArgs<'buf> for T
where
    T: Declarative<'buf>,
{
    type Argument = ();
    fn parse_with(buffer: &'buf [u8], argument: Self::Argument) -> DeclResult<'buf, Self> {
        Self::parse(buffer)
    }
}

pub trait DeclRead<'buf>: Sized {
    fn decode<T>(&mut self) -> Result<T, Error> 
    where 
        T: Declarative<'buf>;

    fn decode_with<T>(&mut self, T::Argument) -> Result<T, Error> 
    where 
        T: DeclarativeWithArgs<'buf>;

    fn decode_array<T>(&mut self, length: usize) -> Result<Array<'buf, T>, Error>
    where
        T: Declarative<'buf> + StaticEncodingSize
    {
        DeclRead::decode::<Array<T>>(self, (length, ()))
    }

    fn decode_array_with<T>(&mut self, length: usize, argument: T::Argument) -> Result<Array<'buf, T>, Error>
    where
        T: DeclarativeWithArgs<'buf> + StaticEncodingSize
    {
        DeclRead::decode::<Array<T>>(self, (length, argument))
    }
}

impl<'buf> DeclRead<'buf> for &'buf [u8] {
    fn decode<T>(&mut self) -> Result<T, Error> 
    where 
        T: Declarative<'buf>
    {
        let (result, rest) = T::parse(self)?;
        *self = rest;
        Ok(result)
    }

    fn decode_with<T>(&mut self, argument: T::Argument) -> Result<T, Error> 
    where 
        T: DeclarativeWithArgs<'buf>
    {
        let (result, rest) = T::parse_with(self, argument)?;
        *self = rest;
        Ok(result)
    }
}