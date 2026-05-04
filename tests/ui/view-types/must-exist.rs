#![feature(view_types)]
#![allow(unused)]

struct S {
    foo: (),
}

fn f(_: S.{ bar }) {}
//~^ ERROR no field `bar` on type `S`
fn g(_: S.{ foo, bar }) {}
//~^ ERROR no field `bar` on type `S`

fn main() {}
