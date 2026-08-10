#![feature(view_types, view_type_macro)]

use std::view::view_type;

struct Foo {
    bar: (),
    baz: (),
}

impl Foo {
    fn by_val(self: view_type!(Foo.{ bar })) {}
    fn by_ref(self: &view_type!(Foo.{ bar })) {}
    fn by_mut_ref(self: &mut view_type!(Foo.{ bar })) {}
}

fn main() {
    let mut foo = Foo { bar: (), baz: () };
    // foo.by_ref();
    // foo.by_mut_ref();
    foo.by_val();
}
