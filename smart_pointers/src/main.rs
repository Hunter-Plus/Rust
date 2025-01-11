// Store at heap
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// Deref implementation
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// deref coercion
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Drop implementation
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping when out of scope with data {}", self.data);
    }
}

// reference counting Rc<T>
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

use crate::RcList::{RcCons, RcNil};
use std::rc::Rc;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //hello(&(*m)[..]);

    // Dropping
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);

    let aa = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating aa = {}", Rc::strong_count(&aa));
    let bb = RcCons(3, Rc::clone(&aa));
    println!("count after creating bb = {}", Rc::strong_count(&aa));
    {
        let cc = RcCons(4, Rc::clone(&aa));
        println!("count after creating cc = {}", Rc::strong_count(&aa));
    }
    println!(
        "count after cc goes out of scope = {}",
        Rc::strong_count(&aa)
    );
}
