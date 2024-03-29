trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn do_something<T: Foo>(x: T) {
    x.method();
}

fn dynamic_something(x: &Foo) {
    x.method();
}

pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut(),
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    dynamic_something(&x as &Foo);
    do_something(x);
    do_something(y);
}
