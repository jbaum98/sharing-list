#![allow(dead_code)]

use std::borrow::Borrow;

#[derive(Debug)]
enum IntList<'a> {
    Empty,
    Cons(i32, Cowish<'a, IntList<'a>>),
}

#[derive(Debug)]
enum Cowish<'a, T>
where
    T: 'a,
{
    Borrowed(&'a T),
    Owned(Box<T>),
}

impl<'a, T> Borrow<T> for Cowish<'a, T> where T: 'a {
     fn borrow(&self) -> &T {
         match self {
             &Cowish::Borrowed(b) => b,
             &Cowish::Owned(ref o) => o.borrow(),
         }
     }
}

impl<'a, T> Cowish<'a, T> where T: Clone + 'a {
     fn into_owned(self) -> Box<T> {
         match self {
            Cowish::Borrowed(b) => Box::new(b.clone()),
            Cowish::Owned(o) => o,
         }
     }
}

use IntList::*;

fn main() {
    // let x = Cons(1, &Cons(2, &Cons(3, &Empty)));
    // println!("{:?}", x)
}
