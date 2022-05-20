#![allow(dead_code)]
#![allow(unused_variables)]
use object_oriented::AveragedCollection;

fn main() {
    println!("\nMost people have encountered OOP.\nHere's an example object in Rust:");
    let mut collection = AveragedCollection::new();
    collection.add(3); collection.add(7); collection.add(5);
    println!(
        "{:?}",
        collection
    );

    println!("\nCheck out the code for an example of trait objects and more.")
}

// Trait Objects
use object_oriented::gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use object_oriented::gui::{Button, Screen};

fn gui_example() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ],
    };
    
    screen.run();
}

// More Object-Oriented Style:
use object_oriented::blog1::Post;

fn blog_example() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}