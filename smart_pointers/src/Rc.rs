use std::rc::Rc;

enum List{
    Cons(i32, Rc<List>),
    Nil,
}

use crate::Rc::List::{Cons, Nil};

pub fn main() {
    let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// cloning the Rc increases the reference count
// Rc<T> enables multiple owners of the same data; 
// Rc<T> keeps track of the number of references to a value which determines whether or not a value is still in use.

// Box<T> allows immutable or mutable borrows checked at compile time;
// Rc<T> allows only immutable borrows checked at compile time;
// RefCell<T> allows immutable or mutable borrows checked at runtime.