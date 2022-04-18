use std::collections::HashMap;

fn main() {
    // Vector
    let mut v: Vec<i32> = Vec::new();
    // using the vec! macro
    // let v  = vec![1, 2, 3]; 

    // adding new elements
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Orange"), 50);

    //or
    let teams = vec![String::from("Blue"), String::from("Orange")];
    let initial_scores = vec![10, 50];
    let zip_scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let _score = zip_scores.get(&team_name); //Notice the Option type

    // Iterating through a HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Using entry/or_insert to update a value based on an old value:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    } // mutable reference goes out of scope -- safe

    println!("{:?}", map);
}

fn _vec_borrowing() {
    let mut v  = vec![1, 2, 3];

    let _second = &v[1]; //immutable borrow

    v.push(6); // mutable borrow

    // The following will not compile:
    // println!("The second entry to v: {}", _second);

    println!("The second entry to v: {}", &v[1]);
}

// Exercises

fn _median_mode() {

}

fn _pig_latin() {

}

fn _add_to_department() {
    
}