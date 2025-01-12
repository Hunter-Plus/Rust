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

use std::{cell::RefCell, ops::Deref};

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
use std::rc::{Rc, Weak};

// multiple owners
#[derive(Debug)]
enum RefList {
    RefCons(Rc<RefCell<i32>>, Rc<RefList>),
    RefNil,
}

use crate::RefList::{RefCons, RefNil};

// using weak<T> to avoid reference cycle

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

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

    let value = Rc::new(RefCell::new(5));

    let aaa = Rc::new(RefCons(Rc::clone(&value), Rc::new(RefNil)));

    let bbb = RefCons(Rc::new(RefCell::new(3)), Rc::clone(&aaa));

    let ccc = RefCons(Rc::new(RefCell::new(4)), Rc::clone(&aaa));

    *value.borrow_mut() += 10;

    println!("aaa after = {aaa:?}");
    println!("bbb after = {bbb:?}");
    println!("ccc after = {ccc:?}");

    // avoiding reference cycles

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!(
        "leaf parent when init = {:?}",
        leaf.parent.borrow().upgrade()
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "leaf parent after assignment = {:?}",
            leaf.parent.borrow().upgrade()
        );
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
