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
fn _why_slice() {
    let mut s = String::from("hello world");

    let _word = _first_word(&s); //word = 5

    s.clear(); //'word' is now meaningless, as we changed s
    // so, we want something that will _refer_ to a portion of s
    // here is where slices come in
}

 fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
     
    s.len()
 }

 fn _slice_eg() {
    let s  = String::from("hello world");

    let _hello = _first_word_slice(&s);
    let _world = &s[6..11];

    //s.clear(); //error!
    // mutable borrow occurs here

    println!("The first word is: {}", _hello);
    // immutable borrow used later
 }

 fn _first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
 }

 /*
  * More general slices can occur 
  * -- anything with a first element and a length (vectors, Ch8.)
  * For example,
  *     let a = [1, 2, 3, 4, 5];
  *     let slice = &a[1..3];
  *     assert_eq!(slice, &[2, 3]); 
  */