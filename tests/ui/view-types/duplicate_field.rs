//@ known-bug: unknown
//@ run-pass

#![feature(view_types)]
#![allow(unused)]

struct Foo {
    bar: usize,
}

struct Pair(usize);

fn f(
    _foo: &mut Foo.{ bar, bar },
    _pair: &mut Pair.{ 0, 0 },
) {}

impl Foo {
    fn f(&mut self.{ bar, bar }) {}
}

impl Pair {
    fn f(&mut self.{ 0, 0 }) {}
}

fn main() {}
