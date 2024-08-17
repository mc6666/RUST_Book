struct Baz;

// trait
trait Foo {
    fn foo(&self);
}

// trait inheritance
trait FooBar : Foo {
    fn foobar(&self);
}

// 實作
impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}

fn main() {
    let x = Baz;
    x.foo();
    x.foobar();
}
