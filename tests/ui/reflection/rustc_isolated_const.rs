#![feature(const_trait_impl, rustc_attrs)]

#[rustc_isolated_const]
const VAL: usize = {
    <() as Foo>::bar()
    //~^ ERROR
};

const trait Foo {
    fn bar() -> usize {
        todo!()
    }
}

const impl Foo for () {}

fn main() {
    assert_eq!(VAL, 42);
}
