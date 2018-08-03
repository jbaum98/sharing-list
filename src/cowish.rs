use std::borrow::Borrow;

#[derive(Debug)]
pub enum Cowish<'a, T, B>
where
    T: 'a,
{
    Borrowed(&'a T),
    Owned(B),
}

impl<'a, T, B> Borrow<T> for Cowish<'a, T, B>
where
    T: 'a,
    B: Borrow<T>,
{
    fn borrow(&self) -> &T {
        match self {
            Cowish::Borrowed(b) => b,
            Cowish::Owned(o) => o.borrow(),
        }
    }
}

impl<'a, T, B> Cowish<'a, T, B>
where
    T: Into<B> + Clone + 'a,
{
    pub fn into_owned(self) -> B {
        match self {
            Cowish::Borrowed(b) => b.clone().into(),
            Cowish::Owned(o) => o,
        }
    }
}