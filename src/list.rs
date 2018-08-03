use std::borrow::Borrow;
use std::iter::FromIterator;

use cowish::Cowish;

#[derive(Debug)]
pub enum List<'a, T>
where
    T: 'a,
{
    Empty,
    Cons(T, Cowish<'a, List<'a, T>, Box<List<'a, T>>>),
}

impl<'a, T> List<'a, T> {
    pub fn tail(&self) -> &List<'a, T> {
        match self {
            &List::Empty => &List::Empty,
            &List::Cons(_, ref tl) => tl.borrow(),
        }
    }

    pub fn owned_cons(self, el: T) -> List<'a, T> {
        return List::Cons(el, Cowish::Owned(Box::new(self)));
    }

    pub fn cons(&'a self, el: T) -> List<'a, T> {
        return List::Cons(el, Cowish::Borrowed(self));
    }
}

impl<'a, T> FromIterator<T> for List<'a, T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut l = List::Empty;
        for x in iter {
            l = l.owned_cons(x);
        }
        return l
    }
}
