#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum MulList {
    MulCons(i32, Rc<MulList>),
    MulNil,
}

use crate::MulList::{MulCons, MulNil};
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, 
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");

    
    let a = Rc::new(MulCons(5, Rc::new(MulCons(10, Rc::new(MulNil)))));
    let b = MulCons(3, Rc::clone(&a));
    let c = MulCons(4, Rc::clone(&a));

    let a = Rc::new(MulCons(5, Rc::new(MulCons(10, Rc::new(MulNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = MulCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = MulCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
