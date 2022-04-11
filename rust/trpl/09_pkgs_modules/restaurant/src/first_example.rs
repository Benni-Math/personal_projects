mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn server_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::server_order();
    }

    fn cook_order() {}

    // Structs can be public but have private variables inside
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast, if we make an enum public, 
    // all of its variants are then public:
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// We can use 'use' to simplify our module paths:
use first_example::front_of_house::hosting;
// use self::front_of_house::hosting;

// Keep in mind: when bringing functions into scope with use
// always bring in the parent module -- this makes is clear
// what the function is.
// In contrast, when bringing in structs, enums, and other items
// specify the full path, like so:
//
//      use std::collections::HashMap;
//
//      fn main() {
//          let mut map = HashMap::new();
//          map.insert(1, 2);
//      }
//
// There are cases where you still want to bring in the parent path,
// for example, when two types have the same name. This can also be
// solved with the 'as' keyword:
//
//      use std::fmt::Result;
//      use std::io::Result as IoResult;
//
// To make new paths created by 'use' useable by others, we have 'pub use':
//
//      pub use crate::front_of_house::hosting;

// Nested Paths:
//      use std::cmp::Ordering;
//      use std::io;
// is equivalent to:
//      use std::{cmp::Ordering, io};
// and
//      use std::io;
//      use std::io::Write;
// is equivalent to:
//      use std::io::{self, Write};

pub fn eat_at_restaurant() {
    // Absolute path
    first_example::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Ordering breakfast (testing pub on struct)
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Customer changes their mind
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if uncommented
    //  b/c the fruit is not public inside of Breakfast
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Shorter module paths with use:
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}