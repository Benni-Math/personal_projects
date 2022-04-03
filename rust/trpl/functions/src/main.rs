fn main() {
    let x = five();
    another_function(x);

    print_labeled_measurements(x, 'h');

    fun_with_expressions();

    let x = plus_one(plus_one(five()) - five());
    another_function(x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn fun_with_expressions() {
    let y = {
        let x = five();
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 //Don't put a semi-colon here
}

fn plus_one(x: i32) -> i32 {
    x + 1 //No semicolon on expressions r
}