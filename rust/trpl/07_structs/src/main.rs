struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    // Initializing and building the User struct
    let user1_email = String::from("benni@bu.edu");
    let user1_name = String::from("Benni");
    let user1 = build_user(user1_email, user1_name);

    if user1.active {
        println!("Welcome {}!", user1.username);
        println!("\tEmail: {}", user1.email);
        println!("\tSign-in Count: {}", user1.sign_in_count);
    } 

    // Struct update syntax
    let user2 = User {
        email: String::from("ben-new-email@gmail.com"),
        ..user1
    };

    if user2.active {
        println!("Welcome {}!", user2.username);
        println!("\tEmail: {}", user2.email);
        println!("\tSign-in Count: {}", user2.sign_in_count);
    } 
}

fn build_user(email: String, username: String) -> User {
    User { 
        email,
        username,
        active: true, 
        sign_in_count: 1 
    }
}