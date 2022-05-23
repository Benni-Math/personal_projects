use std::boxed::Box;

// Using Box for a recursive data type - cons list
#[derive(Debug)]
enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

use self::ConsList::{ Cons, Nil };

pub fn cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("Checkout this cons list:\n{:?}", list);
}

// Using Box to implement a Linked List
// struct LinkedList {
//     head: Option<Link>,
// }

// type Link = Box<Node>;

// struct Node {
//     value: i32,
//     next: Option<Link>,
// }

// Implementing our own Box-type to understand Deref
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

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn coerced_deref() {
    let m = MyBox::new(String::from("Rust"));
    println!("Deref coercion example:");
    hello(&m);
}