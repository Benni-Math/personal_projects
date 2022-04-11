// Enums allow restrictions on what can be set
// For pattern matching and enumerating options

// First pattern (bad):
/**
 *  enum IpAddrKind {
 *      V4,
 *      V6,
 *  }
 * 
 *  struct IpAddr {
 *      kind: IpAddrKind,
 *      address: String,
 *  }
 * 
 *  let home = IpAddr {
 *      kind: IpAddrKind::V4,
 *      address: String::from("127.0.0.1")
 *  }
 * 
 *  let loopback = IpAddr {
 *      kind: IpAddrKind::V6,
 *      address: String::from("::1")
 *  }
 * 
 */

// Second pattern (good):
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// The Option<T> enum  (included in the prelude)
// enum Option<T> {
//     None,
//     Some(T),
// }

// The match Control Flow Construct
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Matching with Option<t>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("\nV4 IP address: {:?}", home);
    // dbg!(home);
    println!("V6 IP address: {:?}", loopback);
    // dbg!(loopback);

    // Matching Coins
    let lincoln = Coin::Penny;
    println!("\nA penny is worth {} cents", value_in_cents(lincoln));
    let jefferson = Coin::Nickel;
    println!("A Nickel is worth {} cents", value_in_cents(jefferson));
    let roosevelt = Coin::Dime;
    println!("A dime is worth {} cents", value_in_cents(roosevelt));
    let washington = Coin::Quarter;
    println!("A quarter is worth {} cents", value_in_cents(washington));

    //Using the Option<T> enum
    // let some_string = Some("a string");
    let some_number = Some(5);

    let absent_number: Option<i32> = None;

    let six = plus_one(some_number);
    let none = plus_one(absent_number);

    println!("\nMatching with Option<T>: {:?}, {:?}", six, none);

    // If-let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // or
    let mut count = 0;
    let coin = Coin::Quarter;
    if let Coin::Penny = coin {
        println!("Lucky penny!");
    } else {
        count += value_in_cents(coin);
    }
    println!("Total change in your pocket: {}", count);
}

// Catch-all patterns
fn _dice_game() {
    let dice_roll: u8 = 9;
    match dice_roll {
        3 => _add_fancy_hat(),
        7 => _remove_fancy_hat(),
        _ => _reroll(),
    }
    
    // Or
    match dice_roll {
        3 => _add_fancy_hat(),
        7 => _remove_fancy_hat(),
        other => _move_player(other),
    }
}

// empty function for illustration
fn _add_fancy_hat() {}
fn _remove_fancy_hat() {}
fn _reroll() {}
fn _move_player(_other: u8) {}