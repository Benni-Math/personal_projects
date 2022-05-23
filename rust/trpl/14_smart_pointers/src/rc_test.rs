use std::rc::Rc;

#[derive(Debug)]
enum ConsList {
    Cons(i32, Rc<ConsList>),
    Nil,
}

use self::ConsList::{ Cons, Nil };

pub fn many_cons_lists() {
    println!("Here is a demonstration of reference counting:");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}