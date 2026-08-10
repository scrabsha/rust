//@ run-pass

#![feature(view_types, view_type_macro)]
#![allow(unused)]

use std::view::view_type;

struct Foo {
    a: (),
    b: (),
}

impl Foo {
    fn by_val(self: view_type!(Foo.{ a })) {}
    fn by_ref(self: &view_type!(Foo.{ a })) {}
    fn by_ref_ref(self: &&view_type!(Foo.{ a })) {}
    fn by_mut_ref(self: &mut view_type!(Foo.{ a })) {}

    fn boxed(self: Box<view_type!(Foo.{ a })>) {}
}

fn main() {}
