// https://doc.rust-jp.rs/rust-by-example-ja/flow_control/if_let.html

enum Foo {
  Bar,
  Baz,
  Qux(u32)
}

fn main() {
  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  if let Foo::Bar = a {
    println!("a is foobar");
  }

  if let Foo::Bar = {
    println!("b is foobar");
  }

  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }

  if let Foo:Qux(value @ 100) = c {
    println!("c is one hundred");
  }
}