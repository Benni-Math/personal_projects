use std::{sync::{Mutex, MutexGuard}, time::Duration, thread};

use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

pub fn deadlock_demo() {
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                if i_thread % 2 == 0 {
                    let _guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("Thread {} locked mutex1 and will try to lock mutex2, after a nap !", i_thread);

                    thread::sleep(Duration::from_millis(10));

                    let _guard = MUTEX2.lock().unwrap();
                } else {
                    let _guard = MUTEX2.lock().unwrap();

                    println!("Thread {} locked mutex2 and will try to lock mutex1", i_thread);

                    // Here is the deadlock!
                    let _guard = MUTEX1.lock().unwrap();
                }
            }
        }));
    }

    // Wait
    for child in children {
        let _ = child.join();
    }

    println!("This is not printed");
}