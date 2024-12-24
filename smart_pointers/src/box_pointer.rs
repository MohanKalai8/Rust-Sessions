// Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> to be treated like a reference.

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;
use List::{Cons, Nil};

pub fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let p = 6;
    let q = MyBox::new(p);
    assert_eq!(6, p);
    assert_eq!(6, *q);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

pub fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Defining our own Smart Pointer

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Box<T> enables immutable or mutable owners of the same data;
// Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> to be treated like a reference.