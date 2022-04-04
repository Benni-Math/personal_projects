fn main() {
    reference();
    println!();
    mut_ref();
}

/*
 * _Borrowing_ - the act of creating a reference 
 */
fn reference() {
    let s1 = String::from("hello");

    //makes sure ownership isn't passed to function
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable references
// Can't have multiple mutable references (see lifetimes)
fn mut_ref() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
 * Example of a dangling reference 
 *  fn main() {
 *      let reference_to_nothing = dangle();
 *  }
 * 
 *  fn dangle() -> &String {
 *      let s  = String::from("hello");
 * 
 *      &s
 *  }
 * 
 *  This reference is dangling because s moves out of scope
 *  This will not compile
 *  Instead do this:
 *  fn no_dangle() {
 *      let s = String::from("hello");
 * 
 *      s
 *  }
 *  In this case, ownership is transferred, so there is no problem
 */

 // Slices
 fn first_word(s: &String) -> usize {
     let bytes = s.as_bytes();

     for (i, &item) in bytes.iter().enumerate() {
         if item == b' ' {
             return i;
         }
     }
     
     s.len()
 }