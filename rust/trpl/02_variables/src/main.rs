use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("3 hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    // This value is immutable
    let x = 5;
    // Using let allows us to shadow 
    // (and 'change' the value of x)
    let x = x + 1;
    /*
     * Same effect as:
     *      let mut x = 5;
     *      x = x + 1;
     * but slightly different...
     * For example:
     *      let spaces = "   ";
     *      let spaces = spaces.len();
     * We are creating a new variable with let
     * and notice that spaces is first a string
     * but then a number, whereas
     *      let mut spaces = "   ";
     *      spaces = spaces.len();
     * gives 'mismatched type' error --> can't mutate types
     */
    {
        // Shadowing in an inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    // The value here is still 6
    println!("The value of x is: {}", x);
    /*
     * Integers have unsigned (u32) and signed (i32) types 
     * from 8-128 bits -- defaults to i32
     * (and an isize/usize for the computers arch-size) 
     * 
     * Integer literals can be suffixed with type, ex:
     *      57u8
     * In addition, there are other literals as well
     * and the underscore is invisible/only for visual aid
     *      Decimal     |   98_222      //==98222
     *      Hex         |   0xff        //==255
     *      Octal       |   0o77        //==63
     *      Binary      |   0b1111_000  //==120
     *      Byte        |   b'A'        //u8 only
     * 
     * Finally, there are two floats: f32 and f64
     * defaults to f64
     */

    // Tuples:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    //Using index values
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // Arrays:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5];

    // Testing runtime error with out of bound index:
    array_test(a);
}

fn array_test(a : [i32; 5]) {
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
