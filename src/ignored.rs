use std::marker::PhantomData;
use declarative::Declarative;
use declarative::DeclResult;

pub struct Ignore(PhantomData<T>);

impl<'buf, T> Declarative<'buf> for Ignored<T>
where
    T: Declarative<'buf>
{
    fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
        if buffer.
        Ok(Ignore(PhantomD))
    }
}