#![allow(dead_code)]

use std::borrow::Borrow;

#[derive(Debug)]
enum IntList<'a> {
    Empty,
    Cons(i32, Cowish<'a, IntList<'a>, Box<IntList<'a>>>),
}

#[derive(Debug)]
enum Cowish<'a, T, B>
where T: 'a,
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
    fn into_owned(self) -> B {
        match self {
            Cowish::Borrowed(b) => b.clone().into(),
            Cowish::Owned(o) => o,
        }
    }
}

fn main() {
    // let x = Cons(1, &Cons(2, &Cons(3, &Empty)));
    // println!("{:?}", x)
}
