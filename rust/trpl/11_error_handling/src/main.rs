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

    let f = match f {
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
}
