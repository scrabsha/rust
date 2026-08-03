#![feature(view_types, view_type_macro)]

use std::view::view_type;

struct Foo {
    bar: (),
    baz: (),
}

impl Foo {
    fn meow(self: view_type!(Foo.{ bar })) {
        println!("wagu wagu");
    }
}

fn main() {
    let foo = Foo { bar: (), baz: () };
    foo.meow();
}
