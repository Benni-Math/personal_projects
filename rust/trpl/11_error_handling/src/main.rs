// This is what Result looks like:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let v = vec![1, 2, 3];

    // This will cause a panic! error
    // v[99];
    
    println!("The first element in v: {}", &v[0]);

    // Using the Results type:
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    }; // Notice that Ok and Err are already in scope (from the prelude)

    // Instead of nesting matches, we can use closures with unwrap_or_else (see later chapters)
    // Or, we can use .unwrap():
    //      let f = File::open("hello.txt").unwrap();
    // This will execute a panic! if the Result is an Err, and will 'unwrap' the Ok if there is no error
    // Similarly, we can use .expect():
    //      let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // This allows us to more easily label errors
}
