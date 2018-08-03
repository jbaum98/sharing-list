use std::borrow::Borrow;

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
            Cowish::Borrowed(b) => b,
            Cowish::Owned(o) => o.borrow(),
        }
    }
}

impl<'a, T, O> Cowish<'a, T, O>
where
    T: Into<O> + Clone + 'a,
{
    pub fn into_owned(self) -> O {
        match self {
            Cowish::Borrowed(b) => b.clone().into(),
            Cowish::Owned(o) => o,
        }
    }
}
