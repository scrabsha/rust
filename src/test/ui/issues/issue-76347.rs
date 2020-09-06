pub enum Foo {
    A(i32),
}

impl Foo {
    pub const A: Self = Foo::A(0);
    //~^ ERROR duplicate definition with name `A` [E0592]
}

fn main() {}
