use std::borrow::{Borrow, ToOwned};

use self::Cowish::*;

#[derive(Debug)]
pub enum Cowish<'a, T, O>
where
    T: 'a,
{
    Borrowed(&'a T),
    Owned(O),
}

impl<'a, T, O> Borrow<T> for Cowish<'a, T, O>
where
    T: 'a,
    O: Borrow<T>,
{
    fn borrow(&self) -> &T {
        match self {
            Borrowed(b) => b,
            Owned(o) => o.borrow(),
        }
    }
}

impl<'a, T, O> Cowish<'a, T, O>
where
    T: ToOwned<Owned=O> + 'a,
    O: Borrow<T>,
{
    pub fn into_owned(self) -> O {
        match self {
            Borrowed(b) => b.to_owned(),
            Owned(o) => o,
        }
    }
}
