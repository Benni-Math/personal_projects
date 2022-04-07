// Enums allow restrictions on what can be set
// For pattern matching and slight variations in struct

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
    println!("\nV6 IP address: {:?}", loopback);
    // dbg!(loopback);

    //Using the Option<T> enum
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let six = plus_one(some_number);
    let none = plus_one(absent_number);

    println!("Matching with Option<T>: {:?}, {:?}", six, none);
}