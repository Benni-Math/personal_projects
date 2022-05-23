#![allow(unused_variables)]

use std::thread;
use std::time::Duration;

pub fn thread_demo() {
    // Notice that we capture the threads output with a variable
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // If we put the handle.join().unwrap() here...
    // handle.join().unwrap();
    // It stops the main thread until the spawned thread is finished

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // This will allow the new thread to finish
    handle.join().unwrap();
}

pub fn move_semantics() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}