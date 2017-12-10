use std::marker::PhantomData;
use std::fmt::Debug;
use errors::Error;
use declarative::DeclarativeArgsRead;
use declarative::DeclResult;
use declarative::DeclarativeArgs;
use declarative::StaticEncodingSize;
use declarative::DynamicEncodingSize;

#[derive(Debug)]
pub struct Array<'buf, Item>
where
    Item: Debug,
    Item: DeclarativeArgs<'buf>,
    Item::Argument: Debug,
{
    buffer: &'buf [u8],
    length: usize,
    argument: <Item as DeclarativeArgs<'buf>>::Argument,
    phantom: PhantomData<Item>,
}

impl<'buf, Item> DeclarativeArgs<'buf> for Array<'buf, Item>
where
    Item: Debug,
    Item: DeclarativeArgs<'buf>,
    Item::Argument: Debug,
{
    type Argument = (usize, Item::Argument);
    fn parse_with(
        buffer: &'buf [u8],
        arguments: (usize, Item::Argument),
    ) -> DeclResult<'buf, Self> {
        Ok((
            Array {
                buffer: buffer,
                length: arguments.0,
                argument: arguments.1,
                phantom: PhantomData,
            },
            buffer,
        ))
    }
}

impl<'buf, Item> DynamicEncodingSize for Array<'buf, Item>
where
    Item: Debug,
    Item: StaticEncodingSize + DeclarativeArgs<'buf>,
    Item::Argument: Debug,
{
    fn size(&self) -> usize {
        Item::SIZE * self.length
    }
}

impl<'buf, Item> IntoIterator for Array<'buf, Item>
where
    Item: Debug,
    Item: StaticEncodingSize,
    Item: DeclarativeArgs<'buf>,
    <Item as DeclarativeArgs<'buf>>::Argument: Clone,
    Item::Argument: Debug,
{
    type IntoIter = ArrayIter<'buf, Item>;
    type Item = Result<Item, Error>;
    fn into_iter(self) -> Self::IntoIter {
        ArrayIter {
            buffer: self.buffer,
            length: self.length,
            argument: self.argument,
            cursor: 0usize,
            phantom: PhantomData,
        }
    }
}

pub struct ArrayIter<'buf, Item>
where
    Item: Debug,
    Item: DeclarativeArgs<'buf>,
    <Item as DeclarativeArgs<'buf>>::Argument: Clone,
    Item::Argument: Debug,
{
    buffer: &'buf [u8],
    length: usize,
    argument: <Item as DeclarativeArgs<'buf>>::Argument,
    cursor: usize,
    phantom: PhantomData<Item>,
}

impl<'buf, Item> Iterator for ArrayIter<'buf, Item>
where
    Item: Debug,
    Item: StaticEncodingSize + DeclarativeArgs<'buf>,
    <Item as DeclarativeArgs<'buf>>::Argument: Clone,
    Item::Argument: Debug,
{
    type Item = Result<Item, Error>;
    fn next(&mut self) -> Option<Result<Item, Error>> {
        if self.length <= self.cursor {
            return None;
        }

        self.cursor += 1;
        let dest = self.buffer.parse_with::<Item>(self.argument.clone());
        Some(dest)
    }
}

// TODO:
//  [ ] Implement linear search?
//  [ ] Implement binary search?
//  [ ] Implement .get(usize) interface?
