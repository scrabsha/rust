#![feature(view_types)]
#![allow(unused)]

enum Foo {
    Bar,
    Baz,
}

struct Cat  {
    mrow: (),
    mrrp: (),
}

fn f(_: Foo.{}) {}
//~^ ERROR only structs can be viewed
fn g(_: u8.{}) {}
//~^ ERROR only structs can be viewed
fn h(_: char.{}) {}
//~^ ERROR only structs can be viewed

fn i(_: Cat.{ mrow, mrrp }) {}

fn main() {}
