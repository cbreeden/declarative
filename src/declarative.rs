use errors::Error;
use array::Array;
use std::fmt::Debug;

pub type DeclResult<'buf, T> = Result<(T, &'buf [u8]), Error>;

/// This trait is implemented on types whose encoding size is known _prior_
/// to having been parsed.
pub trait StaticEncodingSize {
    const SIZE: usize;
}

/// This trait is implemented on types whose encoding size is known after
/// they have been parsed.
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

pub trait DeclarativeArgs<'buf>: Sized {
    type Argument;
    fn parse_with(buffer: &'buf [u8], arguments: Self::Argument) -> DeclResult<'buf, Self>;
}

pub trait Declarative<'buf>: Sized {
    fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self>;
}

impl<'buf, T> DeclarativeArgs<'buf> for T
where
    T: Declarative<'buf>,
{
    type Argument = ();
    fn parse_with(buffer: &'buf [u8], arguments: Self::Argument) -> DeclResult<'buf, Self> {
        <Self as Declarative<'buf>>::parse(buffer)
    }
}

pub trait DeclarativeRead<'buf>: Sized {
    fn parse<T: Declarative<'buf>>(&mut self) -> Result<T, Error>;
    fn parse_array<T>(&mut self, length: usize) -> Result<Array<'buf, T>, Error>
    where
        T: Debug + PartialEq,
        T: Declarative<'buf> + StaticEncodingSize;
}

impl<'buf> DeclarativeRead<'buf> for &'buf [u8] {
    fn parse<T: Declarative<'buf>>(&mut self) -> Result<T, Error> {
        let (result, rest) = T::parse(self)?;
        *self = rest;
        Ok(result)
    }

    fn parse_array<T>(&mut self, length: usize) -> Result<Array<'buf, T>, Error>
    where
        T: Debug + PartialEq,
        T: Declarative<'buf> + StaticEncodingSize,
    {
        self.parse_with::<Array<T>>((length, ()))
    }
}

pub trait DeclarativeArgsRead<'buf>: Sized {
    fn parse_with<T>(&mut self, args: T::Argument) -> Result<T, Error>
    where
        T: DeclarativeArgs<'buf>;
}

impl<'buf> DeclarativeArgsRead<'buf> for &'buf [u8] {
    fn parse_with<T>(&mut self, args: T::Argument) -> Result<T, Error>
    where
        T: DeclarativeArgs<'buf>
    {
        let (result, rest) = T::parse_with(self, args)?;
        *self = rest;
        Ok(result)
    }
}
