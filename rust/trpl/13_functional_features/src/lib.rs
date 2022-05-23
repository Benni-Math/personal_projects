mod cache_tools;
mod iterators;

use std::thread;
use std::time::Duration;

// Closures:
use cache_tools::Cacher;
pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Today, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            )
        }
    }
}


use crate::cache_tools::rc_cacher::Cacher as RcCacher;
pub fn generate_workout_rc(intensity: u32, random_number: u32) {
    let mut expensive_result = RcCacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Today, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            )
        }
    }
}


//Iterators:
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }
use std::io;
use std::error::Error;
use iterators::shoe::{ Shoe, shoes_in_size };

pub fn filtering_shoes() -> Result<(), Box<dyn Error>> {
    let available_shoes = vec![
        Shoe {
            size: 8,
            style: String::from("sandals"),
        },
        Shoe {
            size: 8,
            style: String::from("Gucci")
        },
    ];

    println!("Enter your shoe size:");

    let mut shoe_size = String::new();

    io::stdin()
        .read_line(&mut shoe_size)
        .expect("Failed to read line");
    
    let shoe_size: u32 = shoe_size.trim().parse()?;

    let matching_shoes = shoes_in_size(available_shoes, shoe_size);

    if matching_shoes.len() < 1 {
        println!("Sorry, we have no shoes in your size...");
    } else {
        println!("Here are the available shoes in your size:\n {:?}", matching_shoes);
    }

    Ok(())
}