use std::io;

fn main() {
    println!("Breaking outer loops:");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    println!("\nReturning results on break:");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //returns this value
        }
    };

    println!("The result is {}", result);

    println!("\nUsing a while loop:");
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("\nUsing a for loop for an array:");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }

    println!("\nUsing a for loop with a range:");
    println!("Initiate countdown...");
    // We can use .rev() to reverse the range (1..4)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn _test() {
    println!("Choose a number between 0-9");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");
    
    let input: u8 = input
        .trim()
        .parse()
        .expect("Not a number!");
    
    println!("{} {}", input, _test_expression(input));
}


fn _test_expression(x: u8) -> String {
    let x = if x > 5 {
        "> 5"
    } else if x < 5 {
        "< 5"
    } else {
        "== 5"
    };

    String::from(x)
}

// temperature conversion: Fahrenheit <-> Celsius
// fn temp_conversion(temp: String) -> String {

// }

// Fibonacci generator
// fn fibonacci(n: usize) -> usize {
//     if n == 0 {

//     }
// }

// "The Twelve Days of Christmas" generator