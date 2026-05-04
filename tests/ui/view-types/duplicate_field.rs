#![feature(view_types)]
#![allow(unused)]

struct Foo {
    bar: usize,
}

struct Pair(usize);

fn f(
    _foo: &mut Foo.{ bar, bar },
    //~^ ERROR field `bar` is already part of the view
    _pair: &mut Pair.{ 0, 0 },
    //~^ ERROR field `0` is already part of the view
) {}

impl Foo {
    fn f(&mut self.{ bar, bar }) {}
    //~^ ERROR field `bar` is already part of the view
}

impl Pair {
    fn f(&mut self.{ 0, 0 }) {}
    //~^ ERROR field `0` is already part of the view
}

fn main() {}
