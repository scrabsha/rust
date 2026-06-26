#![feature(const_trait_impl, rustc_attrs)]

#[rustc_isolated_const]
const VAL: () = {
    <() as Foo>::bar();
    //~^ ERROR
};

const trait Foo {
    fn bar() {}
}

const impl Foo for () {}

fn main() {}
