#![allow(dead_code)]

use std::iter::Iterator;

mod cowish;
mod list;


use list::List;

fn main() {
    let tl = (1..10).collect::<List<i32>>();
    // We move the tail, so l1 owns it
    let l1 = tl.owned_cons(0);
    // We reference the tail so we can still use it without moving
    let l2 = l1.tail().cons(-1);
    println!("{:?}", l1);
    println!("{:?}", l2);
}
